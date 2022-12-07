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

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRustAdd(t *testing.T) {
	result := Add(1, 3)
	assert.Equal(t, 4, result)
}

func TestRustFib(t *testing.T) {
	result := Fib(7)
	assert.Equal(t, 13, result)
}

func BenchmarkGoAdd(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		add(1, 3)
	}
}

func add(a int, b int) int {
	return a + b
}

func BenchmarkRustAdd(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		Add(1, 3)
	}
}

func BenchmarkGoFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		fib(7)
	}
}

func fib(a int) int {
	if a < 2 {
		return a
	}

	return fib(a-1) + fib(a-2)
}

func BenchmarkRustFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		Fib(7)
	}
}

func BenchmarkCadenceFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		CadenceFib(7)
	}
}
