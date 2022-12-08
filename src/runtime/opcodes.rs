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
    ReturnValue{index: usize},
    Jump{target: usize},
    JumpIfFalse {condition: usize, target: usize},
    IntIntBinOp{typ: IntBinOpType, left_operand: usize, right_operand: usize, result: usize},
    IntBoolBinOp{typ: CmpBinOpType, left_operand: usize, right_operand: usize, result: usize},
    IntMove {from: usize, to: usize},
    IntConstantLoad{index: usize, target: usize},
    GlobalFuncLoad{index: usize, result: usize},
    Call {func_index: usize, arguments: Vec<Argument>, result: usize},
    True{index: usize},
    False{index: usize},
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

macro_rules! define_binops {
    (
        $(
            $constructor:path {

                $(
                    $name:ident, $typ:path
                );* $(;)*
            }
        );* $(;)*
    ) => {
        impl OpCode {
            $(
                $(

                    #[allow(non_snake_case)]
                    pub const fn $name(left_operand: usize, right_operand: usize, result: usize) -> OpCode {
                        $constructor {
                            typ: $typ,
                            left_operand,
                            right_operand,
                            result
                        }
                    }
                )*
            )*
        }
    }
}

define_binops!{
    OpCode::IntIntBinOp {
        IntAdd, IntBinOpType::Add;
        IntSub, IntBinOpType::Sub;
        IntMul, IntBinOpType::Mul;
    };

    OpCode::IntBoolBinOp {
        IntLess,         CmpBinOpType::LT;
        IntLessEqual,    CmpBinOpType::LTE;
        IntEqual,        CmpBinOpType::EQ;
        IntGreater,      CmpBinOpType::GT;
        IntGreaterEqual, CmpBinOpType::GTE;
    }
}

impl OpCode {
    pub fn execute(&self, vm: &mut vm::VM) {
        use OpCode::*;
        use IntBinOpType::*;
        use CmpBinOpType::*;
        match self {
            Return => panic!("not implemented"),
            ReturnValue{index} => {vm.pop_call_frame(*index)}
            Jump{target} => {
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
            &IntConstantLoad{index, target} => {
                let constant = vm.constants[index];
                let int_reg = &mut vm.call_frame().locals.ints;
                int_reg[target] = constant;
            },
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
            &True{index} => {
                vm.call_frame().locals.bools[index] = values::TRUE_VALUE;
            }
            &False{index} => {
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
// pub trait OpCode {
//     fn execute(&self, vm: &mut vm::VM);
// }

// pub struct Return {}

// impl OpCode for Return {
//     fn execute(&self, _: &mut vm::VM) {
//         panic!("not implemented!")
//     }
// }

// pub struct ReturnValue {
//     pub index: usize,
// }

// impl OpCode for ReturnValue {
//     fn execute(&self, vm: &mut vm::VM) {
//         vm.pop_call_frame(self.index);
//     }
// }

// pub struct Jump {
//     pub target: usize,
// }

// impl OpCode for Jump {
//     fn execute(&self, vm: &mut vm::VM) {
//         vm.call_frame().ip = self.target;
//     }
// }

// pub struct JumpIfFalse {
//     pub condition: usize,
//     pub target: usize,
// }

// impl OpCode for JumpIfFalse {
//     fn execute(&self, vm: &mut vm::VM) {
//         let call_frame = vm.call_frame();
//         let bools = &call_frame.locals.bools;

//         let condition = &bools[self.condition];
//         if !condition.value {
//             call_frame.ip = self.target;
//         }
//     }
// }

// pub struct IntAdd {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntAdd {
//     fn execute(&self, vm: &mut vm::VM) {
//         let int_reg = &mut vm.call_frame().locals.ints;
//         let left_number = &int_reg[self.left_operand];
//         let right_number = &int_reg[self.right_operand];
//         int_reg[self.result] = left_number.add(right_number);
//     }
// }

// pub struct IntSubtract {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntSubtract {
//     fn execute(&self, vm: &mut vm::VM) {
//         let int_reg = &mut vm.call_frame().locals.ints;
//         let left_number = &int_reg[self.left_operand];
//         let right_number = &int_reg[self.right_operand];
//         int_reg[self.result] = left_number.subtract(right_number);
//     }
// }

// pub struct IntEqual {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntEqual {
//     fn execute(&self, vm: &mut vm::VM) {
//         let int_reg = &mut vm.call_frame().locals.ints;
//         let left_number = &int_reg[self.left_operand];
//         let right_number = &int_reg[self.right_operand];
//         int_reg[self.result] = left_number.add(right_number);
//     }
// }

// pub struct IntNotEqual {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntNotEqual {
//     fn execute(&self, _: &mut vm::VM) {
//         panic!("not implemented!")
//     }
// }

// pub struct IntLess {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntLess {
//     fn execute(&self, vm: &mut vm::VM) {
//         let call_frame = vm.call_frame();
//         let int_reg = &mut call_frame.locals.ints;
//         let left_number = &int_reg[self.left_operand];
//         let right_number = &int_reg[self.right_operand];

//         call_frame.locals.bools[self.result] = left_number.less(right_number);
//     }
// }

// pub struct IntGreater {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntGreater {
//     fn execute(&self, vm: &mut vm::VM) {
//         let call_frame = vm.call_frame();
//         let int_reg = &mut call_frame.locals.ints;
//         let left_number = &int_reg[self.left_operand];
//         let right_number = &int_reg[self.right_operand];

//         call_frame.locals.bools[self.result] = left_number.greater(right_number);
//     }
// }

// pub struct IntLessOrEqual {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntLessOrEqual {
//     fn execute(&self, _: &mut vm::VM) {
//         panic!("not implemented!")
//     }
// }

// pub struct IntGreaterOrEqual {
//     pub left_operand: usize,
//     pub right_operand: usize,
//     pub result: usize,
// }

// impl OpCode for IntGreaterOrEqual {
//     fn execute(&self, _: &mut vm::VM) {
//         panic!("not implemented!")
//     }
// }

// pub struct IntConstantLoad {
//     pub index: usize,
//     pub target: usize,
// }

// impl OpCode for IntConstantLoad {
//     fn execute(&self, vm: &mut vm::VM) {
//     }
// }

// pub struct True {
//     pub index: usize,
// }

// impl OpCode for True {
//     fn execute(&self, vm: &mut vm::VM) {
//         vm.call_frame().locals.bools[self.index] = values::TRUE_VALUE;
//     }
// }

// pub struct False {
//     pub index: usize,
// }
// impl OpCode for False {
//     fn execute(&self, vm: &mut vm::VM) {
//         vm.call_frame().locals.bools[self.index] = values::FALSE_VALUE;
//     }
// }
// pub struct IntMove {
//     pub from: usize,
//     pub to: usize,
// }

// impl OpCode for IntMove {
//     fn execute(&self, vm: &mut vm::VM) {
//         let int_reg = &mut vm.call_frame().locals.ints;
//         int_reg[self.to] = int_reg[self.from];
//     }
// }

// pub struct GlobalFuncLoad {
//     pub index: usize,
//     pub result: usize,
// }

// impl OpCode for GlobalFuncLoad {
//     fn execute(&self, vm: &mut vm::VM) {
//         let value = vm.globals[self.index];
//         vm.call_frame().locals.funcs[self.result] = Option::Some(value);
//     }
// }

// pub struct Call<'a> {
//     pub func_index: usize,
//     pub arguments: &'a [Argument],
//     pub result: usize,
// }

// impl<'a> OpCode for Call<'a> {
//     fn execute(&self, vm: &mut vm::VM) {
//         let value = vm.call_frame().locals.funcs[self.func_index].as_ref();
//         let func = value.unwrap().function;
//         vm.push_call_frame(func, self.arguments, self.result);
//     }
// }

// pub struct Argument {
//     pub typ: registers::RegisterType,
//     pub index: usize,
// }
