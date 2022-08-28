package leetcode

import (
	"math"
	"testing"
)

func reverse(x int) int {
	result := 0
	for x != 0 {
		result = result*10 + x%10
		x = x / 10
	}
	// 2147483647
	if result < math.MinInt32 || result > math.MaxInt32 {
		return 0
	}

	return result
}
func Test0007(t *testing.T) {
	var tests = []struct {
		input, ans int
	}{
		{123, 321},
		{120, 21},
		{-123, -321},
	}
	for _, test := range tests {
		r := reverse(test.input)
		if r != test.ans {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
