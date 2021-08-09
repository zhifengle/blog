package leetcode;

import leetcode.st.ListNode;

public class S0002AddTwoNumbers {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        int carry = 0;
        int sum = 0;
        ListNode res = new ListNode(0);
        ListNode head = res;
        while (l1 != null || l2 != null || sum > 0) {
            if (l1 != null) {
                sum = sum + l1.val;
                l1 = l1.next;
            }
            if (l2 != null) {
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
}
