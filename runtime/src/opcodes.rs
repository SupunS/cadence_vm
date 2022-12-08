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

use crate::{registers};

pub enum OpCode<'a> {
    Return {},

    ReturnValue {
        index: usize,
    },

    Jump {
        target: usize,
    },

    JumpIfFalse {
        condition: usize,
        target: usize,
    },

    IntAdd {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntSubtract {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntEqual {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntLess {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntLessOrEqual {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntGreater {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntGreaterOrEqual {
        left_operand: usize,
        right_operand: usize,
        result: usize,
    },

    IntConstantLoad {
        index: usize,
        target: usize,
    },

    True {
        index: usize,
    },

    False {
        index: usize,
    },

    IntMove {
        from: usize,
        to: usize,
    },

    GlobalFuncLoad {
        index: usize,
        result: usize,
    },

    Call {
        func_index: usize,
        arguments: &'a [Argument],
        result: usize,
    },
}

pub struct Argument {
    pub typ: registers::RegisterType,
    pub index: usize,
}
