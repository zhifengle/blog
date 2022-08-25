package leetcode

import (
	"reflect"
	"sort"
	"testing"
)

// https://leetcode.com/problems/3sum/

func threeSum(nums []int) [][]int {
	var result [][]int

	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] > 0 {
			break
		}
		// edge: 跳过重复
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		lo := i + 1
		hi := len(nums) - 1
		for lo < hi {
			sum := nums[i] + nums[lo] + nums[hi]
			if sum < 0 {
				lo++
			} else if sum > 0 {
				hi--
			} else {
				result = append(result, []int{nums[i], nums[lo], nums[hi]})
				// edge: 跳过重复
				for lo < hi && nums[lo] == nums[lo+1] {
					lo++
				}
				// edge: 跳过重复
				for lo < hi && nums[hi] == nums[hi-1] {
					hi--
				}
				lo++
				hi--
			}
		}
	}

	return result
}

func Test0003(t *testing.T) {
	var tests = []struct {
		input []int
		ans   [][]int
	}{
		{[]int{-1, 0, 1, 2, -1, -4}, [][]int{{-1, -1, 2}, {-1, 0, 1}}},
	}
	for _, test := range tests {
		r := threeSum(test.input)
		if !reflect.DeepEqual(test.ans, r) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
