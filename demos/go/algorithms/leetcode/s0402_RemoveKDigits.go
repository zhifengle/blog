package leetcode

import (
	"regexp"
	"strings"
)

func removeKdigits(num string, k int) string {
	remain := len(num) - k
	var stack []string

	for _, c := range num {
		s := string(c)
		for k > 0 && len(stack) > 0 && stack[len(stack)-1] > s {
			stack = stack[:len(stack)-1]
			k -= 1
		}
		stack = append(stack, s)
	}
	v := strings.Join(stack[:remain], "")
	r := regexp.MustCompile("^0*")
	v = r.ReplaceAllString(v, "")
	if v == "" {
		v = "0"
	}
	return v
}
