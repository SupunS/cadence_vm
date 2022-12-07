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

use crate::{registers, values, vm};

pub trait OpCode {
    fn execute(&self, vm: &mut vm::VM);
}

pub struct Return {}

impl OpCode for Return {
    fn execute(&self, _: &mut vm::VM) {
        panic!("not implemented!")
    }
}

pub struct ReturnValue {
    pub index: usize,
}

impl OpCode for ReturnValue {
    fn execute(&self, vm: &mut vm::VM) {
        vm.pop_call_frame(self.index);
    }
}

pub struct Jump {
    pub target: usize,
}

impl OpCode for Jump {
    fn execute(&self, vm: &mut vm::VM) {
        vm.call_frame().ip = self.target;
    }
}

pub struct JumpIfFalse {
    pub condition: usize,
    pub target: usize,
}

impl OpCode for JumpIfFalse {
    fn execute(&self, vm: &mut vm::VM) {
        let call_frame = vm.call_frame();
        let bools = &call_frame.locals.bools;

        let condition = &bools[self.condition];
        if !condition.value {
            call_frame.ip = self.target;
        }
    }
}

pub struct IntAdd {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntAdd {
    fn execute(&self, vm: &mut vm::VM) {
        let int_reg = &mut vm.call_frame().locals.ints;
        let left_number = &int_reg[self.left_operand];
        let right_number = &int_reg[self.right_operand];
        int_reg[self.result] = left_number.add(right_number);
    }
}

pub struct IntSubtract {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntSubtract {
    fn execute(&self, vm: &mut vm::VM) {
        let int_reg = &mut vm.call_frame().locals.ints;
        let left_number = &int_reg[self.left_operand];
        let right_number = &int_reg[self.right_operand];
        int_reg[self.result] = left_number.subtract(right_number);
    }
}

pub struct IntEqual {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntEqual {
    fn execute(&self, vm: &mut vm::VM) {
        let int_reg = &mut vm.call_frame().locals.ints;
        let left_number = &int_reg[self.left_operand];
        let right_number = &int_reg[self.right_operand];
        int_reg[self.result] = left_number.add(right_number);
    }
}

pub struct IntNotEqual {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntNotEqual {
    fn execute(&self, _: &mut vm::VM) {
        panic!("not implemented!")
    }
}

pub struct IntLess {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntLess {
    fn execute(&self, vm: &mut vm::VM) {
        let call_frame = vm.call_frame();
        let int_reg = &mut call_frame.locals.ints;
        let left_number = &int_reg[self.left_operand];
        let right_number = &int_reg[self.right_operand];

        call_frame.locals.bools[self.result] = left_number.less(right_number);
    }
}

pub struct IntGreater {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntGreater {
    fn execute(&self, vm: &mut vm::VM) {
        let call_frame = vm.call_frame();
        let int_reg = &mut call_frame.locals.ints;
        let left_number = &int_reg[self.left_operand];
        let right_number = &int_reg[self.right_operand];

        call_frame.locals.bools[self.result] = left_number.greater(right_number);
    }
}

pub struct IntLessOrEqual {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntLessOrEqual {
    fn execute(&self, _: &mut vm::VM) {
        panic!("not implemented!")
    }
}

pub struct IntGreaterOrEqual {
    pub left_operand: usize,
    pub right_operand: usize,
    pub result: usize,
}

impl OpCode for IntGreaterOrEqual {
    fn execute(&self, _: &mut vm::VM) {
        panic!("not implemented!")
    }
}

pub struct IntConstantLoad {
    pub index: usize,
    pub target: usize,
}

impl OpCode for IntConstantLoad {
    fn execute(&self, vm: &mut vm::VM) {
        let constant = vm.constants[self.index];
        let int_reg = &mut vm.call_frame().locals.ints;
        int_reg[self.target] = constant;
    }
}

pub struct True {
    pub index: usize,
}

impl OpCode for True {
    fn execute(&self, vm: &mut vm::VM) {
        vm.call_frame().locals.bools[self.index] = values::TRUE_VALUE;
    }
}

pub struct False {
    pub index: usize,
}
impl OpCode for False {
    fn execute(&self, vm: &mut vm::VM) {
        vm.call_frame().locals.bools[self.index] = values::FALSE_VALUE;
    }
}
pub struct IntMove {
    pub from: usize,
    pub to: usize,
}

impl OpCode for IntMove {
    fn execute(&self, vm: &mut vm::VM) {
        let int_reg = &mut vm.call_frame().locals.ints;
        int_reg[self.to] = int_reg[self.from];
    }
}

pub struct GlobalFuncLoad {
    pub index: usize,
    pub result: usize,
}

impl OpCode for GlobalFuncLoad {
    fn execute(&self, vm: &mut vm::VM) {
        let value = vm.globals[self.index];
        vm.call_frame().locals.funcs[self.result] = Option::Some(value);
    }
}

pub struct Call<'a> {
    pub func_index: usize,
    pub arguments: &'a [Argument],
    pub result: usize,
}

impl<'a> OpCode for Call<'a> {
    fn execute(&self, vm: &mut vm::VM) {
        let value = vm.call_frame().locals.funcs[self.func_index].as_ref();
        let func = value.unwrap().function;
        vm.push_call_frame(func, self.arguments, self.result);
    }
}

pub struct Argument {
    pub typ: registers::RegisterType,
    pub index: usize,
}
