package leetcode

import "testing"

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}
	num := x
	y := 0
	for num != 0 {
		s := num % 10
		y = y*10 + s
		num = num / 10
	}
	return x == y
}

func Test0009(t *testing.T) {
	var tests = []struct {
		input int
		ans   bool
	}{
		{121, true},
		{-121, false},
	}
	for _, test := range tests {
		r := isPalindrome(test.input)
		if r != test.ans {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
