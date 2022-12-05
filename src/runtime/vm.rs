/*
 * Cadence - The resource-oriented smart contract programming language
 *
 * Copyright 2019-2022 Dapper Labs, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::runtime::opcodes::Argument;
use crate::runtime::values::{BoolValue, FunctionValue, IntValue, FALSE_VALUE, INT_ZERO_VALUE};
use crate::runtime::{bbq, registers};

use crate::runtime::bbq::Function;
use crate::runtime::registers::RegisterType;

pub struct VM<'a> {
    // Program   *bbq.Program
    pub globals: Vec<FunctionValue<'a>>,
    pub constants: Vec<IntValue>,
    // functions map[string]*bbq.Function
    pub call_stack: Vec<CallFrame<'a>>,
    pub current_index: usize,

    pub return_value: IntValue,
}

pub struct CallFrame<'a> {
    pub(crate) locals: Registers<'a>,
    function: &'a bbq::Function,
    pub(crate) ip: usize,

    return_to_index: usize,
}

pub struct Registers<'a> {
    pub(crate) ints: Vec<IntValue>,
    pub(crate) bools: Vec<BoolValue>,
    pub(crate) funcs: Vec<Option<FunctionValue<'a>>>,
}

impl<'a> Registers<'a> {
    fn new(function: &'a Function) -> Self {
        Registers {
            ints: vec![INT_ZERO_VALUE; function.local_count.ints],
            bools: vec![FALSE_VALUE; function.local_count.bools],
            funcs: vec![None; function.local_count.funcs],
        }
    }
}

impl<'a> VM<'a> {
    pub fn invoke(&mut self, function: &'a Function, argument: IntValue) -> IntValue {
        // TODO: pass in the function name and look it up from the functions map.

        let mut locals = Registers::new(function);

        locals.ints[0] = argument;

        let call_frame = CallFrame {
            locals: locals,
            function: function,
            ip: 0,
            return_to_index: 0,
        };

        self.call_stack.push(call_frame);

        self.run();

        return self.return_value;
    }

    pub(crate) fn call_frame(&mut self) -> &mut CallFrame<'a> {
        let size = self.call_stack.len() - 1;
        return &mut self.call_stack[size];
    }

    pub(crate) fn run(&mut self) {
        loop {
            if self.call_stack.is_empty() {
                return;
            }

            let mut call_frame = self.call_frame();
            let ip = call_frame.ip;

            if ip >= call_frame.function.code.len() {
                return;
            }

            call_frame.ip += 1;
            call_frame.function.code[ip].execute(self);
        }
    }

    pub(crate) fn push_call_frame(
        &mut self,
        function: &'a Function,
        arguments: &[Argument],
        result_index: usize,
    ) {
        let mut locals = Registers::new(function);

        let current_call_frame = self.call_frame();

        current_call_frame
            .locals
            .copy_arguments_to(&mut locals, arguments);

        let call_frame = CallFrame {
            locals: locals,
            function: function,
            ip: 0,
            return_to_index: result_index,
        };

        self.call_stack.push(call_frame);
    }

    pub(crate) fn pop_call_frame(&mut self, return_value_index: usize) {
        let call_frame = self.call_stack.pop().unwrap();
        let return_value = call_frame.locals.ints[return_value_index];

        if self.call_stack.is_empty() {
            self.return_value = return_value;
            return;
        }

        let parent = self.call_frame();

        // Copy the return value from callee to caller.
        // TODO: Currently assumes the return value is always Integer.
        //  Fix this to copy from/to the correct register based on the return value type.
        parent.locals.ints[call_frame.return_to_index] = return_value;
    }

    fn initialize_constant(&mut self, index: usize) {
        // TODO
    }
}

impl<'a> Registers<'a> {
    fn copy_arguments_to(&self, target_registers: &mut Registers<'a>, arguments: &[Argument]) {
        let mut reg_counts = registers::RegisterCounts {
            ints: 0,
            bools: 0,
            funcs: 0,
        };

        for argument in arguments {
            match argument.typ {
                RegisterType::Int => {
                    target_registers.ints[reg_counts.ints] = self.ints[argument.index];
                    reg_counts.ints += 1;
                }
                RegisterType::Bool => {
                    target_registers.bools[reg_counts.bools] = self.bools[argument.index];
                    reg_counts.bools += 1;
                }
                RegisterType::Func => {
                    target_registers.funcs[reg_counts.funcs] = self.funcs[argument.index];
                    reg_counts.funcs += 1;
                }
            }
        }
    }
}
