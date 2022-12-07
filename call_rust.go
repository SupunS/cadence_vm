package gorust

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

func CDCFib(a int) int {
	return int(C.cdcfib(C.int(a)))
}
