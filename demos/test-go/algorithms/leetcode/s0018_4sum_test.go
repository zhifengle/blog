package leetcode

import (
	"reflect"
	"sort"
	"testing"
)

func fourSum(nums []int, target int) [][]int {
	var result [][]int

	l := len(nums)
	sort.Ints(nums)
	for i := 0; i < l-3; i++ {
		if i > 0 && nums[i-1] == nums[i] {
			continue
		}
		for j := i + 1; j < l; j++ {
			if j > i+1 && nums[j-1] == nums[j] {
				continue
			}
			lo := j + 1
			hi := l - 1
			for lo < hi {
				sum := nums[i] + nums[j] + nums[lo] + nums[hi]
				if sum < target {
					lo++
				} else if sum > target {
					hi--
				} else {
					result = append(result, []int{nums[i], nums[j], nums[lo], nums[hi]})
					for lo < hi && nums[lo] == nums[lo+1] {
						lo++
					}
					for lo < hi && nums[hi] == nums[hi-1] {
						hi--
					}
					lo++
					hi--
				}
			}
		}
	}

	return result
}

func Test0018(t *testing.T) {
	var tests = []struct {
		input  []int
		target int
		ans    [][]int
	}{
		{[]int{1, 0, -1, 0, -2, 2}, 0, [][]int{{-2, -1, 1, 2}, {-2, 0, 0, 2}, {-1, 0, 0, 1}}},
	}
	for _, test := range tests {
		r := fourSum(test.input, test.target)
		if !reflect.DeepEqual(test.ans, r) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
