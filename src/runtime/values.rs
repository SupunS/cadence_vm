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
use derive_more::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, From, Into};

pub(crate) trait Value { }

/*
*  IntValue
*/
#[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[mul(forward)]
pub struct IntValue {
    pub value: isize,
}

pub(crate) const INT_ZERO_VALUE: IntValue = IntValue { value: 0 };

impl Value for IntValue {}

/*
*  BoolValue
*/

#[derive(Clone, Copy, From, Into)]
pub struct BoolValue {
    pub value: bool,
}

impl Value for BoolValue {}

pub(crate) const TRUE_VALUE: BoolValue = BoolValue { value: true };

pub(crate) const FALSE_VALUE: BoolValue = BoolValue { value: false };

/*
*  FunctionValue
*/
pub struct FunctionValue<'a> {
    pub function: &'a bbq::Function,
}

impl<'a> Value for FunctionValue<'a> {}

impl<'a> Clone for FunctionValue<'a> {
    fn clone(&self) -> Self {
        FunctionValue {
            function: self.function,
        }
    }
}

impl<'a> Copy for FunctionValue<'a> {}
