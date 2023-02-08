package leetcode

import (
	"test_go/algorithms/structures"
	"testing"
)

// https://leetcode.com/problems/merge-two-sorted-lists/

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	dummy := &ListNode{}
	p := dummy
	for list1 != nil || list2 != nil {
		if list2 == nil {
			p.Next = list1
			break
		}
		if list1 == nil {
			p.Next = list2
			break
		}
		if list1.Val <= list2.Val {
			p.Next = list1
			list1 = list1.Next
		} else {
			p.Next = list2
			list2 = list2.Next
		}
		p = p.Next
	}
	return dummy.Next
}
func Test0021(t *testing.T) {
	tests := []struct {
		list1, list2, ans []int
	}{
		{[]int{1, 2, 4}, []int{1, 3, 4}, []int{1, 1, 2, 3, 4, 4}},
		{[]int{}, []int{0}, []int{0}},
		{[]int{}, []int{}, []int{}},
	}
	for _, test := range tests {
		r := structures.List2Ints(mergeTwoLists(structures.Ints2List(test.list1), structures.Ints2List(test.list2)))
		if !EqualInts(r, test.ans) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
