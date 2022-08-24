package leetcode

import (
	"test_go/algorithms/structures"
	"testing"
)

type ListNode = structures.ListNode

// https://github.com/halfrost/LeetCode-Go/tree/master/leetcode/0002.Add-Two-Numbers

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	total := 0
	carry := 0
	r := &ListNode{}
	head := r
	for l1 != nil || l2 != nil || total > 0 {
		if l1 != nil {
			total += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			total += l2.Val
			l2 = l2.Next
		}
		if total >= 10 {
			total -= 10
			carry = 1
		}
		head.Next = &ListNode{Val: total}
		head = head.Next
		total = carry
		carry = 0
	}
	return r.Next
}

func Test0002(t *testing.T) {
	var tests = []struct {
		l1, l2, ans []int
	}{
		{[]int{2, 4, 3}, []int{5, 6, 4}, []int{7, 0, 8}},
		{[]int{1}, []int{9, 9, 9, 9, 9}, []int{0, 0, 0, 0, 0, 1}},
	}
	for _, test := range tests {
		r := structures.List2Ints(addTwoNumbers(structures.Ints2List(test.l1), structures.Ints2List(test.l2)))
		if !EqualInts(test.ans, r) {
			//if !reflect.DeepEqual(test.ans, r) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
