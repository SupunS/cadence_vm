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


use crate::runtime::{registers, values::{self, BoolValue}, vm};

#[derive(Clone, Copy)]
pub struct Argument {
    pub typ: registers::RegisterType,
    pub index: usize,
}
macro_rules! binop {
    ($name:ident) => {$name{left_operand: usize, right_operand: usize, result: usize}}
}
#[derive(Clone, PartialEq, Eq)]
pub struct BinOpArgs{left_operand: usize, right_operand: usize, result: usize}
pub enum OpCode {
    Return,
    ReturnValue(usize),
    Jump(usize),
    JumpIfFalse {condition: usize, target: usize},
    IntIntBinOp{typ: IntBinOpType, left_operand: usize, right_operand: usize, result: usize},
    IntBoolBinOp{typ: CmpBinOpType, left_operand: usize, right_operand: usize, result: usize},
    IntMove {from: usize, to: usize},
    GlobalFuncLoad{index: usize, result: usize},
    Call {func_index: usize, arguments: Vec<Argument>, result: usize},
    True(usize),
    False(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IntBinOpType {
    Add, 
    Sub,
    Mul,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CmpBinOpType {
    EQ,
    LT,
    LTE,
    GT,
    GTE,
}

impl OpCode {
    pub fn execute(&self, vm: &mut vm::VM) {
        use OpCode::*;
        use IntBinOpType::*;
        use CmpBinOpType::*;
        match self {
            Return => panic!("not implemented"),
            ReturnValue(idx) => {vm.pop_call_frame(*idx)}
            Jump(target) => {
                vm.call_frame().ip = *target;
            }
            &JumpIfFalse{condition: condition_idx, target}  => {
                let call_frame = vm.call_frame();
                let bools = &call_frame.locals.bools;
                let condition = &bools[condition_idx];
                if !condition.value {
                    call_frame.ip = target;
                }
            }
            &IntMove{from, to} => {
                let int_reg = &mut vm.call_frame().locals.ints;
                int_reg[to] = int_reg[from];
            }
            &GlobalFuncLoad{index, result} => {
                let value = vm.globals[index];
                vm.call_frame().locals.funcs[result] = Some(value);
            }
            Call { func_index, arguments, result } => {
                let value = vm.call_frame().locals.funcs[*func_index];
                let func = value.unwrap().function;
                vm.push_call_frame(func, &arguments, *result);
            },
            &True(index) => {
                vm.call_frame().locals.bools[index] = values::TRUE_VALUE;
            }
            &False(index) => {
                vm.call_frame().locals.bools[index] = values::FALSE_VALUE;
            }
            &IntIntBinOp { typ, left_operand, right_operand, result } => { 
                let int_reg = &mut vm.call_frame().locals.ints;
                let lhs = int_reg[left_operand];
                let rhs = int_reg[right_operand];
                int_reg[result] =  match typ {
                    Add => lhs + rhs,
                    Sub => lhs - rhs,
                    Mul => lhs * rhs,
                };
            }
            &IntBoolBinOp{typ, left_operand, right_operand, result} => {
                let call_frame = vm.call_frame();
                let int_reg = &mut call_frame.locals.ints;
                let lhs = int_reg[left_operand];
                let rhs = int_reg[right_operand];
                call_frame.locals.bools[result] = BoolValue::from(match typ {
                    LT  => lhs <  rhs,
                    LTE => lhs <= rhs,
                    EQ  => lhs == rhs,
                    GTE => lhs >= rhs,
                    GT  => lhs >  rhs,
                });
            }
        }
    }
}