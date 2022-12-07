package gorust

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAdd(t *testing.T) {
	result := Add(1, 3)
	assert.Equal(t, 4, result)
}

func TestFib(t *testing.T) {
	result := Fib(7)
	assert.Equal(t, 13, result)
}

func BenchmarkRustAdd(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Add(1, 3)
	}
}

func BenchmarkGoAdd(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		add(1, 3)
	}
}

func add(a int, b int) int {
	return a + b
}

func BenchmarkRustFib(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Fib(7)
	}
}

func BenchmarkGoFib(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		fib(7)
	}
}

func BenchmarkCDCFib(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		CDCFib(7)
	}
}

func fib(a int) int {
	if a < 2 {
		return a
	}

	return fib(a-1) + fib(a-2)
}
