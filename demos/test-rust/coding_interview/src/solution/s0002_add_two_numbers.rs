#![allow(unused_imports)]
#![allow(dead_code)]

use crate::base_st::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut total = 0;
        let mut r = ListNode::new(0);
        let mut head = &mut r;

        while l1.is_some() || l2.is_some() || total > 0 {
            if l1.is_some() {
                total = total + l1.as_ref()?.val;
                l1 = l1?.next;
            }
            if l2.is_some() {
                total = total + l2.as_ref()?.val;
                l2 = l2?.next;
            }
            if total >= 10 {
                total -= 10;
                carry = 1;
            }
            head.next = Some(Box::new(ListNode {
                val: total,
                next: None,
            }));
            head = head.next.as_mut()?;

            total = carry;
            carry = 0;
        }

        r.next
    }
}
