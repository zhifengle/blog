import { ListNode } from '../links';

function addTwoNumbers(
  l1: ListNode<number> | null,
  l2: ListNode<number> | null
): ListNode<number> | null {
  let carry = 0;
  let sum = 0;
  let res = new ListNode(0);
  let head = res;
  while (l1 || l2 || sum > 0) {
    if (l1) {
      sum = sum + l1.val;
      l1 = l1.next;
    }
    if (l2) {
      sum = sum + l2.val;
      l2 = l2.next;
    }
    if (sum >= 10) {
      carry = 1;
      sum -= 10;
    }
    head.next = new ListNode(sum);
    head = head.next;

    sum = carry;
    carry = 0;
  }

  return res.next;
}
