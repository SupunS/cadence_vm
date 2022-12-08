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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegisterCounts {
    pub ints: usize,
    pub bools: usize,
    pub funcs: usize,
}

impl RegisterCounts {
    fn next_index(mut self, register_type: RegisterType) -> usize {
        let index: usize;
        match register_type {
            RegisterType::Int => {
                index = self.ints;
                self.ints += 1;
            }
            RegisterType::Bool => {
                index = self.bools;
                self.bools += 1;
            }
            RegisterType::Func => {
                index = self.funcs;
                self.funcs += 1;
            }
        }

        return index;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterType {
    Int,
    Bool,
    Func,
}
