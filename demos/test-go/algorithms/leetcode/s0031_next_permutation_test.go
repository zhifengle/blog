package leetcode

import (
	"testing"
)

// https://leetcode.com/problems/next-permutation/

func nextPermutation(nums []int) {
	l := len(nums)
	i := l - 1
	j := l - 1

	for i > 0 && nums[i-1] >= nums[i] {
		i--
	}
	if i > 0 {
		for j > i && nums[j] <= nums[i-1] {
			j--
		}
		nums[i-1], nums[j] = nums[j], nums[i-1]
	}

	j = l - 1
	for i < j {
		nums[i], nums[j] = nums[j], nums[i]
		i++
		j--
	}
}

func Test0031(t *testing.T) {
	var tests = []struct {
		input, ans []int
	}{
		{[]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 5, 4}},
		{[]int{5, 4, 3, 2, 1}, []int{1, 2, 3, 4, 5}},
		// 2 3 1;    i-1  2 这里是递减了
		// 找到 j; j要比 i-1 大
		{[]int{2, 3, 1}, []int{3, 1, 2}},
	}
	for _, test := range tests {
		nextPermutation(test.input)
		if !EqualInts(test.input, test.ans) {
			t.Errorf("expect %v, got %v", test.ans, test.input)
		}
	}
}
