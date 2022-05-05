import { ListNode } from '../links';

function removeNthFromEnd(
  head: ListNode<number> | null,
  n: number
): ListNode<number> | null {
  if (!head) {
    return null;
  }
  let len = 0;
  let dummyHead = new ListNode(0, head);
  let p = dummyHead;
  while (p.next) {
    len += 1;
    p = p.next;
  }
  let idx = len - n;
  p = dummyHead;
  for (let i = 0; i < idx; i++) {
    p = p.next;
  }
  const next = p.next.next;
  p.next = next;

  return dummyHead.next;
}
