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

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use runtime::bbq::Function;
use runtime::opcodes::{
    Argument, OpCode,
};
use runtime::registers;
use runtime::values::{FunctionValue, IntValue};
use runtime::vm::VM;

fn bench_cadence_recursive_fib(c: &mut Criterion) {
    let func = Function {
        local_count: registers::RegisterCounts {
            ints: 9,
            bools: 1,
            funcs: 2,
        },
        code: vec![
            // if n < 2
            OpCode::IntConstantLoad {
                index: 0,
                target: 1,
            },
            OpCode::IntLess {
                left_operand: 0,
                right_operand: 1,
                result: 0,
            },
            OpCode::JumpIfFalse {
                condition: 0,
                target: 4,
            },
            // then return n
            OpCode::ReturnValue { index: 0 },
            // fib(n - 1)
            OpCode::IntConstantLoad {
                index: 1,
                target: 2,
            },
            OpCode::IntSubtract {
                left_operand: 0,
                right_operand: 2,
                result: 3,
            },
            OpCode::GlobalFuncLoad {
                index: 0,
                result: 0,
            },
            OpCode::Call {
                func_index: 0,
                arguments: &[Argument {
                    typ: registers::RegisterType::Int,
                    index: 3,
                }],
                result: 4,
            },
            // fib(n - 2)
            OpCode::IntConstantLoad {
                index: 2,
                target: 5,
            },
            OpCode::IntSubtract {
                left_operand: 0,
                right_operand: 5,
                result: 6,
            },
            OpCode::GlobalFuncLoad {
                index: 0,
                result: 1,
            },
            OpCode::Call {
                func_index: 1,
                arguments: &[Argument {
                    typ: registers::RegisterType::Int,
                    index: 6,
                }],
                result: 7,
            },
            // return sum
            OpCode::IntAdd {
                left_operand: 4,
                right_operand: 7,
                result: 8,
            },
            OpCode::ReturnValue { index: 8 },
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

    let n = IntValue { value: 14 };

    c.bench_function("cadence recursive fib 14", |b| {
        b.iter(|| vm.invoke(&func, black_box(n)))
    });
}

fn bench_cadence_imperative_fib(c: &mut Criterion) {
    let func = Function {
        local_count: registers::RegisterCounts {
            ints: 11,
            bools: 1,
            funcs: 0,
        },
        code: vec![
            // var fib1 = 1
            OpCode::IntConstantLoad {
                index: 0,
                target: 1,
            },
            OpCode::IntMove { from: 1, to: 2 },
            // var fib1 = 1
            OpCode::IntConstantLoad {
                index: 1,
                target: 3,
            },
            OpCode::IntMove { from: 3, to: 4 },
            // var fibonacci = fib1
            OpCode::IntMove { from: 2, to: 5 },
            // var i = 2
            OpCode::IntConstantLoad {
                index: 2,
                target: 6,
            },
            OpCode::IntMove { from: 6, to: 7 },
            // while i < n
            OpCode::IntLess {
                left_operand: 7,
                right_operand: 0,
                result: 0,
            },
            OpCode::JumpIfFalse {
                condition: 0,
                target: 17,
            },
            // fibonacci = fib1 + fib2
            OpCode::IntAdd {
                left_operand: 2,
                right_operand: 4,
                result: 8,
            },
            OpCode::IntMove { from: 8, to: 5 },
            // fib1 = fib2
            OpCode::IntMove { from: 4, to: 2 },
            // fib2 = fibonacci
            OpCode::IntMove { from: 5, to: 4 },
            // i = i + 1
            OpCode::IntConstantLoad {
                index: 3,
                target: 9,
            },
            OpCode::IntAdd {
                left_operand: 7,
                right_operand: 9,
                result: 10,
            },
            OpCode::IntMove { from: 10, to: 7 },
            // continue loop
            OpCode::Jump { target: 7 },
            // return fibonacci
            OpCode::ReturnValue { index: 5 },
        ],
    };

    let mut vm = VM {
        constants: vec![
            IntValue { value: 1 },
            IntValue { value: 1 },
            IntValue { value: 2 },
            IntValue { value: 1 },
        ],
        call_stack: vec![],
        globals: vec![FunctionValue { function: &func }],
        current_index: 0,
        return_value: IntValue { value: 0 },
    };

    let n = IntValue { value: 14 };

    c.bench_function("cadence imperative fib 14", |b| {
        b.iter(|| vm.invoke(&func, black_box(n)))
    });
}

fn bench_rust_fib(c: &mut Criterion) {
    c.bench_function("rust fib 14", |b| b.iter(|| fibonacci(black_box(14))));
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

criterion_group!(
    benches,
    bench_cadence_recursive_fib,
    bench_cadence_imperative_fib,
    bench_rust_fib,
);

criterion_main!(benches,);
