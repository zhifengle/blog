import { ListNode } from '../list-node';

function mergeTwoLists(
  list1: ListNode | null,
  list2: ListNode | null
): ListNode | null {
  const dummyHead = new ListNode(0);
  let head = dummyHead;
  while (list1 || list2) {
    if (list1 === null) {
      head.next = list2;
      break;
    }
    if (list2 === null) {
      head.next = list1;
      break;
    }
    if (list1.val <= list2.val) {
      head.next = list1;
      list1 = list1.next;
    } else {
      head.next = list2;
      list2 = list2.next;
    }
    head = head.next;
  }
  return dummyHead.next;
}

function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
  if (lists.length === 0) {
    return null;
  }
  if (lists.length === 1) {
    return lists[0];
  }
  if (lists.length === 2) {
    return mergeTwoLists(lists[0], lists[1]);
  }
  const mid = lists.length >> 1;
  // ?? 可以传索引优化一些内存。
  const l1 = lists.slice(0, mid);
  const l2 = lists.slice(mid, lists.length);
  return mergeTwoLists(mergeKLists(l1), mergeKLists(l2));
}
