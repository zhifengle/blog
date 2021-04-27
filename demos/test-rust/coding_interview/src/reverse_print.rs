#![allow(unused_imports)]
#![allow(dead_code)]
/// 06

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut list = head;

    while let Some(ph) = list {
        result.push(ph.val);
        list = ph.next;
    }
    result.reverse();
    result
}
