package leetcode

import (
	"strings"
)

func removeDuplicateLetters(s string) string {
	m := make(map[string]int)
	set := make(map[string]bool)
	var stack []string
	for i := 0; i < len(s); i++ {
		c := string(s[i])
		if _, prs := m[c]; prs {
			m[c] += 1
		} else {
			m[c] = 1
		}
	}

	for _, n := range s {
		c := string(n)
		if _, prs := set[c]; !prs {
			for len(stack) > 0 && c < stack[len(stack)-1] && m[stack[len(stack)-1]] > 0 {
				delete(set, stack[len(stack)-1])
				stack = stack[:len(stack)-1]
			}
			set[c] = true
			stack = append(stack, c)
		}
		m[c] -= 1
	}

	return strings.Join(stack, "")
}
