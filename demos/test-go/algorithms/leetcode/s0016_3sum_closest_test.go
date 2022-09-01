package leetcode

import (
	"sort"
	"testing"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func threeSumClosest(nums []int, target int) int {
	l := len(nums)
	closest := 0
	if l <= 3 {
		for _, n := range nums {
			closest += n
		}
		return closest
	}
	sort.Ints(nums)
	closest = nums[0] + nums[1] + nums[2]
	minDistance := abs(target - closest)
	for i := 0; i < l; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		lo := i + 1
		hi := l - 1
		for lo < hi {
			cur := nums[i] + nums[lo] + nums[hi]
			dis := abs(target - cur)
			if minDistance > dis {
				closest = cur
				minDistance = dis
			}
			if cur > target {
				hi--
			} else if cur < target {
				lo++
			} else {
				return target
			}
		}
	}
	return closest
}

func Test0016(t *testing.T) {
	var tests = []struct {
		input       []int
		target, ans int
	}{
		//{[]int{-1, 2, 1, -4}, 1, 2},
		//{[]int{0, 0, 0}, 1, 0},
		{[]int{1, 1, 1, 1}, 0, 3},
	}
	for _, test := range tests {
		r := threeSumClosest(test.input, test.target)
		if test.ans != r {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
