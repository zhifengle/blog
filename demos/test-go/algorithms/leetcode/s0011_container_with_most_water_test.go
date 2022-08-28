package leetcode

import (
	"testing"
)

// https://leetcode.com/problems/container-with-most-water/

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}
func maxArea2(height []int) int {
	water := 0
	lo := 0
	hi := len(height) - 1
	for lo < hi {
		h := min(height[lo], height[hi])
		water = max(water, h*(hi-lo))
		if height[lo] < height[hi] {
			lo++
		} else {
			hi--
		}
	}
	return water
}

func maxArea(height []int) int {
	water := 0
	current := 0
	tall := 0
	width := 0
	lo := 0
	hi := len(height) - 1

	for lo < hi {
		width = hi - lo
		if height[lo] > height[hi] {
			tall = height[hi]
			hi--
		} else {
			tall = height[lo]
			lo++
		}
		current = tall * width
		if current > water {
			water = current
		}
	}

	return water
}

func Test0011(t *testing.T) {
	var tests = []struct {
		input []int
		ans   int
	}{
		{[]int{1, 8, 6, 2, 5, 4, 8, 3, 7}, 49},
		{[]int{1, 1}, 1},
	}
	for _, test := range tests {
		r := maxArea(test.input)
		if test.ans != r {
			//if !reflect.DeepEqual(test.ans, r) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
