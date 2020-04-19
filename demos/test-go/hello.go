package main

import (
	_ "test_go/algorithms"
	_ "test_go/basic"
)

func adder() func(int) int {
	sum := 0
	return func(x int) int {
		sum += x
		return sum
	}
}
func main() {
}
