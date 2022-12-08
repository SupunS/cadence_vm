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

use crate::bbq::Function;
use crate::opcodes::{Argument, OpCode};
use crate::registers::{RegisterCounts, RegisterType};
use crate::values::{BoolValue, FALSE_VALUE, FunctionValue, INT_ZERO_VALUE, IntValue, TRUE_VALUE};

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

pub struct CallFrame<'a> {
    pub(crate) locals: Registers<'a>,
    function: &'a Function<'a>,
    pub(crate) ip: usize,

    return_to_index: usize,
}

impl<'a> CallFrame<'a> {
    fn new(function: &'a Function, locals: Registers<'a>, return_to_index: usize) -> Self {
        CallFrame {
            function,
            locals,
            return_to_index,
            ip: 0,
        }
    }
}

pub struct VM<'a> {
    // Program   *bbq.Program
    pub globals: Vec<FunctionValue<'a>>,
    pub constants: Vec<IntValue>,
    // functions map[string]*bbq.Function
    pub call_stack: Vec<CallFrame<'a>>,
    pub current_index: usize,

    pub return_value: IntValue,
}

impl<'a> VM<'a> {
    pub fn invoke(&mut self, function: &'a Function, argument: IntValue) -> IntValue {
        // TODO: pass in the function name and look it up from the functions map.

        let mut locals = Registers::new(function);
        locals.ints[0] = argument;

        let call_frame = CallFrame::new(function, locals, 0);
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
            match call_frame.function.code[ip] {
                OpCode::Return { .. } => panic!("not implemented"),
                OpCode::ReturnValue { index } => self.opcode_return_value(index),
                OpCode::Jump { target } => self.opcode_jump(target),
                OpCode::JumpIfFalse { condition, target } => self.opcode_int_jump_if_false(condition, target),
                OpCode::IntAdd { left_operand, right_operand, result } => self.opcode_int_add(left_operand, right_operand, result),
                OpCode::IntSubtract { left_operand, right_operand, result } => self.opcode_int_subtract(left_operand, right_operand, result),
                OpCode::IntEqual { left_operand, right_operand, result } => self.opcode_int_add(left_operand, right_operand, result),
                OpCode::IntLess { left_operand, right_operand, result } => self.opcode_int_less(left_operand, right_operand, result),
                OpCode::IntLessOrEqual { .. } => panic!("not implemented"),
                OpCode::IntGreater { left_operand, right_operand, result } => self.opcode_int_greater(left_operand, right_operand, result),
                OpCode::IntGreaterOrEqual { .. } => panic!("not implemented"),
                OpCode::IntConstantLoad { index, target } => self.opcode_int_const_load(index, target),
                OpCode::True { index } => self.opcode_true(index),
                OpCode::False { index } => self.opcode_false(index),
                OpCode::IntMove { from, to } => self.opcode_int_move(from, to),
                OpCode::GlobalFuncLoad { index, result } => self.opcode_global_func_load(index, result),
                OpCode::Call { func_index, arguments, result } => self.opcode_call(func_index, arguments, result),
            }
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

        let call_frame = CallFrame::new(function, locals, result_index);
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


    fn opcode_return_value(&mut self, index: usize) {
        self.pop_call_frame(index);
    }

    fn opcode_jump(&mut self, target: usize) {
        self.call_frame().ip = target;
    }

    fn opcode_int_jump_if_false(&mut self, condition: usize, target: usize) {
        let call_frame = self.call_frame();
        let bools = &call_frame.locals.bools;

        let condition = &bools[condition];
        if !condition.value {
            call_frame.ip = target;
        }
    }

    fn opcode_int_add(&mut self, left_operand: usize, right_operand: usize, result: usize) {
        let int_reg = &mut self.call_frame().locals.ints;
        let left_number = &int_reg[left_operand];
        let right_number = &int_reg[right_operand];
        int_reg[result] = left_number.add(right_number);
    }

    fn opcode_int_subtract(&mut self, left_operand: usize, right_operand: usize, result: usize) {
        let int_reg = &mut self.call_frame().locals.ints;
        let left_number = &int_reg[left_operand];
        let right_number = &int_reg[right_operand];
        int_reg[result] = left_number.subtract(right_number);
    }

    fn opcode_int_equal(&mut self, left_operand: usize, right_operand: usize, result: usize) {
        let int_reg = &mut self.call_frame().locals.ints;
        let left_number = &int_reg[left_operand];
        let right_number = &int_reg[right_operand];
        int_reg[result] = left_number.add(right_number);
    }

    fn opcode_int_less(&mut self, left_operand: usize, right_operand: usize, result: usize) {
        let call_frame = self.call_frame();
        let int_reg = &mut call_frame.locals.ints;
        let left_number = &int_reg[left_operand];
        let right_number = &int_reg[right_operand];

        call_frame.locals.bools[result] = left_number.less(right_number);
    }

    fn opcode_int_greater(&mut self, left_operand: usize, right_operand: usize, result: usize) {
        let call_frame = self.call_frame();
        let int_reg = &mut call_frame.locals.ints;
        let left_number = &int_reg[left_operand];
        let right_number = &int_reg[right_operand];

        call_frame.locals.bools[result] = left_number.greater(right_number);
    }

    fn opcode_int_const_load(&mut self, index: usize, target: usize) {
        let constant = self.constants[index];
        let int_reg = &mut self.call_frame().locals.ints;
        int_reg[target] = constant;
    }

    fn opcode_true(&mut self, index: usize) {
        self.call_frame().locals.bools[index] = TRUE_VALUE;
    }


    fn opcode_false(&mut self, index: usize) {
        self.call_frame().locals.bools[index] = FALSE_VALUE;
    }

    fn opcode_int_move(&mut self, from: usize, to: usize) {
        let int_reg = &mut self.call_frame().locals.ints;
        int_reg[to] = int_reg[from];
    }

    fn opcode_global_func_load(&mut self, index: usize, result: usize) {
        let value = self.globals[index];
        self.call_frame().locals.funcs[result] = Some(value);
    }

    fn opcode_call(&mut self, func_index: usize, arguments: &[Argument], result: usize) {
        let value = self.call_frame().locals.funcs[func_index].as_ref();
        let func = value.unwrap().function;
        self.push_call_frame(func, arguments, result);
    }
}

impl<'a> Registers<'a> {
    fn copy_arguments_to(&self, target_registers: &mut Registers<'a>, arguments: &[Argument]) {
        let mut reg_counts = RegisterCounts {
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
