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

use runtime::{add, cdcfib, fib};

#[test]
fn add_function() {
    let result = add(1 as i32, 2 as i32);
    assert_eq!(result, 3 as i32)
}

#[test]
fn fib_function() {
    let result = fib(14 as i32);
    assert_eq!(result, 377 as i32)
}

#[test]
fn cdc_fib_function() {
    let result = cdcfib(14 as i32);
    assert_eq!(result, 377 as i32)
}
