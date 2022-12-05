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

use crate::runtime::bbq;
use crate::runtime::opcodes::Argument;
use crate::runtime::values::{BoolValue, FunctionValue, IntValue, Value, FALSE_VALUE};

use crate::runtime::bbq::Function;

pub struct VM<'a> {
    // Program   *bbq.Program
    pub globals: Vec<FunctionValue<'a>>,
    pub constants: Vec<IntValue>,
    // functions map[string]*bbq.Function
    pub call_stack: Vec<CallFrame<'a>>,
    pub current_index: usize,
}

pub struct CallFrame<'a> {
    pub(crate) locals: Registers<'a>,
    function: &'a bbq::Function,
    pub(crate) ip: usize,

    pub(crate) return_value_index: usize,
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
            ints: vec![IntValue { value: 0 }; function.local_count.ints],
            bools: vec![FALSE_VALUE; function.local_count.bools],
            funcs: vec![None; function.local_count.funcs],
        }
    }
}

impl<'a> VM<'a> {
    pub(crate) fn invoke(&mut self, function: &'a Function, argument: IntValue) -> IntValue {
        // TODO: pass in the function name and look it up from the functions map.

        let mut locals = Registers::new(function);

        locals.ints.push(argument);

        let call_frame = CallFrame {
            locals: locals,
            function: function,
            ip: 0,
            return_to_index: 0,

            // TODO:
            return_value_index: 8,
        };

        self.call_stack.push(call_frame);

        self.run();

        let call_frame = self.call_frame();
        let result = call_frame.locals.ints[call_frame.return_value_index];

        return result;
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
        let locals = Registers::new(function);

        let current_call_frame = self.call_frame();

        current_call_frame.locals.copy_arguments(&locals, arguments);

        let call_frame = CallFrame {
            locals: locals,
            function: function,
            ip: 0,
            return_value_index: 0,
            return_to_index: result_index,
        };

        self.call_stack.push(call_frame);
    }

    pub(crate) fn pop_call_frame(&mut self, return_value_index: usize) {
        if self.call_stack.len() == 1 {
            self.call_frame().return_value_index = return_value_index;
            return;
        }

        let call_frame = self.call_stack.pop().unwrap();
        let parent = self.call_frame();

        // Copy the return value from callee to caller.
        // TODO: Currently assumes the return value is always Integer.
        //  Fix this to copy from/to the correct register based on the return value type.

        let return_value = call_frame.locals.ints[return_value_index];
        let return_to_index = call_frame.return_to_index;

        parent.locals.ints[return_to_index] = return_value;
    }

    fn initialize_constant(&mut self, index: usize) {
        // TODO
    }
}

impl<'a> Registers<'a> {
    fn copy_arguments(&self, x: &Registers, arguments: &[Argument]) {}
}
