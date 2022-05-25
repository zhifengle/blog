import { ListNode } from '../list-node';

function swapPairs(head: ListNode | null): ListNode | null {
  const dummyHead = new ListNode(0);
  dummyHead.next = head;
  let current = dummyHead;
  while (current.next !== null && current.next.next !== null) {
    const first = current.next;
    const second = current.next.next;

    // 1 -> 2 -> 3 -> 4

    // 1 -> 3 -> 4
    first.next = second.next;
    // 2 -> 1 -> 3 -> 4
    second.next = first;
    // 修正 current
    current.next = second;

    current = current.next.next;
  }
  return dummyHead.next;
}
