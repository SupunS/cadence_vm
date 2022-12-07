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

package cadence_vm

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: ./runtime/libruntime.a -ldl
#include "./runtime/runtime.h"
*/
import "C"

func Add(a int, b int) int {
	var result C.int = C.add(C.int(a), C.int(b))
	return int(result)
}

func Fib(a int) int {
	return int(C.fib(C.int(a)))
}

func CadenceFib(a int) int {
	return int(C.cdcfib(C.int(a)))
}
