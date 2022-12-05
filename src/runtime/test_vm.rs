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

use crate::runtime::bbq::Function;
use crate::runtime::opcodes::{
    Argument, Call, GlobalFuncLoad, IntAdd, IntConstantLoad, IntLess, IntSubtract, JumpIfFalse,
    ReturnValue,
};
use crate::runtime::registers;
use crate::runtime::values::{FunctionValue, IntValue};
use crate::runtime::vm::VM;

#[test]
fn test_vm() {
    let func = Function {
        local_count: registers::RegisterCounts {
            ints: 9,
            bools: 1,
            funcs: 2,
        },
        code: vec![
            // if n < 2
            Box::new(IntConstantLoad {
                index: 0,
                target: 1,
            }),
            Box::new(IntLess {
                left_operand: 0,
                right_operand: 1,
                result: 0,
            }),
            Box::new(JumpIfFalse {
                condition: 0,
                target: 4,
            }),
            // then return n
            Box::new(ReturnValue { index: 0 }),
            // fib(n - 1)
            Box::new(IntConstantLoad {
                index: 1,
                target: 2,
            }),
            Box::new(IntSubtract {
                left_operand: 0,
                right_operand: 2,
                result: 3,
            }),
            Box::new(GlobalFuncLoad {
                index: 0,
                result: 0,
            }),
            Box::new(Call {
                func_index: 0,
                arguments: &[Argument {
                    typ: registers::RegisterType::Int,
                    index: 3,
                }],
                result: 4,
            }),
            // fib(n - 2)
            Box::new(IntConstantLoad {
                index: 2,
                target: 5,
            }),
            Box::new(IntSubtract {
                left_operand: 0,
                right_operand: 5,
                result: 6,
            }),
            Box::new(GlobalFuncLoad {
                index: 0,
                result: 1,
            }),
            Box::new(Call {
                func_index: 1,
                arguments: &[Argument {
                    typ: registers::RegisterType::Int,
                    index: 6,
                }],
                result: 7,
            }),
            // return sum
            Box::new(IntAdd {
                left_operand: 4,
                right_operand: 7,
                result: 8,
            }),
            Box::new(ReturnValue { index: 8 }),
        ],
    };

    let mut vm = VM {
        constants: vec![
            IntValue { value: 2 },
            IntValue { value: 1 },
            IntValue { value: 2 },
        ],
        call_stack: vec![],
        globals: vec![FunctionValue { function: &func }],
        current_index: 0,
        return_value: IntValue { value: 0 },
    };

    let result = vm.invoke(&func, IntValue { value: 7 });

    assert_eq!(result.value, 13);
}
