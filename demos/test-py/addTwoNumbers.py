class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def add_two_numbers(self, l1: ListNode, l2: ListNode):
    carry = 0
    total = 0
    r = ListNode()
    head = r

    while l1 or l2 or total > 0:
        if l1:
            total += l1.val
            l1 = l1.next
        if l2:
            total += l2.val
            l2 = l2.next
        if total >= 10:
            carry = 1
            total -= 10
        head.next = ListNode(total, None)
        head = head.next
        total = carry
        carry = 0

    return r.next
