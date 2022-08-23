package leetcode

import (
	"testing"
)

// 一些参考。
// https://github.com/halfrost/LeetCode-Go

func TestName(t *testing.T) {
	//fmt.Println(removeDuplicateLetters("bcabc"))
}

func Test402(t *testing.T) {
	var tests = []struct {
		num, ans string
		k        int
	}{
		{"1432219", "1219", 3},
		{"1001", "1", 1},
	}
	for _, tt := range tests {
		r := removeKdigits(tt.num, tt.k)
		if tt.ans != r {
			t.Errorf("expect %s, got %s", tt.ans, r)
		}
	}
}
