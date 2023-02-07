package leetcode

import (
	"test_go/algorithms/structures"
	"testing"
)

// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

func removeNthFromEnd2(head *ListNode, n int) *ListNode {
	l := 0
	p := head
	for p.Next != nil {
		l++
		p = p.Next
	}
	// 5   2
	// 2   1
	dummy := &ListNode{}
	dummy.Next = head
	p = dummy
	l = l - n + 1
	for p.Next != nil && l > 0 {
		p = p.Next
		l--
	}
	if l == 0 && p.Next != nil {
		p.Next = p.Next.Next
	}
	return dummy.Next
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	slow := head
	fast := head
	for i := 0; i < n; i++ {
		if fast == nil {
			return head
			break
		}
		fast = fast.Next
	}
	if fast == nil {
		return head.Next
	}
	for fast.Next != nil {
		fast = fast.Next
		slow = slow.Next
	}
	slow.Next = slow.Next.Next
	return head
}

func Test0019(t *testing.T) {
	tests := []struct {
		head []int
		n    int
		ans  []int
	}{
		{[]int{1, 2, 3, 4, 5}, 2, []int{1, 2, 3, 5}},
		{[]int{1, 2, 3, 4, 5}, 5, []int{2, 3, 4, 5}},
		{[]int{1, 2, 3, 4, 5}, 8, []int{1, 2, 3, 4, 5}},
		{[]int{1}, 1, []int{}},
		{[]int{1, 2}, 1, []int{1}},
	}
	for _, test := range tests {
		r := structures.List2Ints(removeNthFromEnd(structures.Ints2List(test.head), test.n))
		if !EqualInts(r, test.ans) {
			t.Errorf("expect %v, got %v", test.ans, r)
		}
	}
}
