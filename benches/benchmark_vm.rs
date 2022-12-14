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

use cadence_vm::runtime::bbq::Function;
use cadence_vm::runtime::opcodes::{
    Argument, Call, GlobalFuncLoad, IntAdd, IntConstantLoad, IntLess, IntMove, IntSubtract, Jump,
    JumpIfFalse, ReturnValue,
};
use cadence_vm::runtime::registers;
use cadence_vm::runtime::values::{FunctionValue, IntValue};
use cadence_vm::runtime::vm::VM;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_cadence_recursive_fib(c: &mut Criterion) {
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

    let n = IntValue { value: 7 };

    c.bench_function("cadence recursive fib 7", |b| {
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
            Box::new(IntConstantLoad {
                index: 0,
                target: 1,
            }),
            Box::new(IntMove { from: 1, to: 2 }),
            // var fib1 = 1
            Box::new(IntConstantLoad {
                index: 1,
                target: 3,
            }),
            Box::new(IntMove { from: 3, to: 4 }),
            // var fibonacci = fib1
            Box::new(IntMove { from: 2, to: 5 }),
            // var i = 2
            Box::new(IntConstantLoad {
                index: 2,
                target: 6,
            }),
            Box::new(IntMove { from: 6, to: 7 }),
            // while i < n
            Box::new(IntLess {
                left_operand: 7,
                right_operand: 0,
                result: 0,
            }),
            Box::new(JumpIfFalse {
                condition: 0,
                target: 17,
            }),
            // fibonacci = fib1 + fib2
            Box::new(IntAdd {
                left_operand: 2,
                right_operand: 4,
                result: 8,
            }),
            Box::new(IntMove { from: 8, to: 5 }),
            // fib1 = fib2
            Box::new(IntMove { from: 4, to: 2 }),
            // fib2 = fibonacci
            Box::new(IntMove { from: 5, to: 4 }),
            // i = i + 1
            Box::new(IntConstantLoad {
                index: 3,
                target: 9,
            }),
            Box::new(IntAdd {
                left_operand: 7,
                right_operand: 9,
                result: 10,
            }),
            Box::new(IntMove { from: 10, to: 7 }),
            // continue loop
            Box::new(Jump { target: 7 }),
            // return fibonacci
            Box::new(ReturnValue { index: 5 }),
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

    let n = IntValue { value: 7 };

    c.bench_function("cadence imperative fib 7", |b| {
        b.iter(|| vm.invoke(&func, black_box(n)))
    });
}

fn bench_rust_fib(c: &mut Criterion) {
    c.bench_function("rust fib 7", |b| b.iter(|| fibonacci(black_box(7))));
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
