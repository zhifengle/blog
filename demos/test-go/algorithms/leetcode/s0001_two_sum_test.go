package leetcode

import (
	"reflect"
	"sort"
	"testing"
)

// https://leetcode.com/problems/two-sum/
// https://books.halfrost.com/leetcode/ChapterFour/0001~0099/0001.Two-Sum/

func twoSum(nums []int, target int) []int {
	// 指定容量比自动增长的  map[int]int{} 要快
	m := make(map[int]int, len(nums)) // 指定了容量
	// m := make(map[int]int)
	//m := map[int]int{}
	// Rust 的写法
	// for (i, num) in nums.iter().enumerate()
	for i, num := range nums {
		// if let Some(val) = m.get(target-num)
		if val, ok := m[target-num]; ok {
			return []int{i, val}
		}
		m[num] = i
	}
	return nil
}

func Test0001(t *testing.T) {
	var tests = []struct {
		nums, ans []int
		target    int
	}{
		{[]int{3, 2, 4}, []int{1, 2}, 6},
		{[]int{2, 7, 11, 15}, []int{0, 1}, 9},
	}
	for _, tt := range tests {
		r := twoSum(tt.nums, tt.target)
		sort.Ints(r)
		if !reflect.DeepEqual(r, tt.ans) {
			t.Errorf("expect %v, got %v", tt.ans, r)
		}
	}
}
