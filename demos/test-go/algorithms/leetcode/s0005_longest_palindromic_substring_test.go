package leetcode

import "testing"

// https://leetcode.com/problems/longest-palindromic-substring/

func findStr(s string, left, right int) string {
	l := len(s)
	for left >= 0 && right < l && s[left] == s[right] {
		left--
		right++
	}
	// left 移动了一下。需要加回来
	return s[left+1 : right]
}

// 测试后 4ms
func longestPalindrome2(s string) string {
	if len(s) <= 1 {
		return s
	}

	str := ""
	longest := ""
	for i := 0; i < len(s)-1; i++ {
		str = findStr(s, i, i)
		if len(str) > len(longest) {
			longest = str
		}
		str = findStr(s, i, i+1)
		if len(str) > len(longest) {
			longest = str
		}
	}
	return longest
}

// 7ms
func longestPalindrome(s string) string {
	strLen := len(s)
	if strLen <= 1 {
		return s
	}
	start := 0
	end := 0
	for i := 0; i < strLen; i++ {
		right := i
		left := i
		for right+1 < strLen && s[left] == s[right+1] {
			right++
		}
		// 这种比较就不存在多移动的情况
		for right+1 < strLen && left > 0 && s[right+1] == s[left-1] {
			right++
			left--
		}
		if right-left > end-start {
			end = right
			start = left
		}
	}
	return s[start : end+1]
}

func Test0005(t *testing.T) {
	var tests = []struct {
		input, ans string
	}{
		{"babad", "bab"},
		{"cbbd", "bb"},
		{"a", "a"},
		{"", ""},
	}
	for _, test := range tests {
		got := longestPalindrome(test.input)
		if got != test.ans {
			t.Errorf("expect %v, got %v", test.ans, got)
		}
	}
}
