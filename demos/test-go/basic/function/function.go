package main

import (
	"errors"
	"fmt"
)

// 声明
type operate func(x, y int) int

// 方案1。
func calculate(x int, y int, op operate) (int, error) {
	if op == nil {
		return 0, errors.New("invalid operation")
	}
	return op(x, y), nil
}

func main() {
	x, y := 12, 23
	op := func(x, y int) int {
		return x + y
	}
	result, err := calculate(x, y, op)
	// error: nil
	fmt.Printf("The result: %d (error: %v)\n",
		result, err)
}
