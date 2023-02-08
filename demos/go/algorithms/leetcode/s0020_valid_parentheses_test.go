package leetcode

import "testing"

// https://leetcode.com/problems/valid-parentheses/

func isValid(s string) bool {
	// 奇数个直接 false
	if len(s)%2 != 0 {
		return false
	}

	parens := map[rune]rune{
		'(': ')',
		'{': '}',
		'[': ']',
	}

	// 这种会存在动态分配的问题。更慢
	// var stack []rune

	// 指定了预留空间
	stack := make([]rune, 0, len(s))

	for _, c := range s {
		if _, ok := parens[c]; ok {
			stack = append(stack, c)
		} else {
			// go 没有  stack.pop()
			if len(stack) == 0 || parens[stack[len(stack)-1]] != c {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}

	return len(stack) == 0
}

func Test0020(t *testing.T) {
	tests := []struct {
		input string
		ans   bool
	}{
		{"()", true},
	}
	for _, test := range tests {
		r := isValid(test.input)
		if test.ans != r {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
