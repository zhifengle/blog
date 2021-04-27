#include <iostream>

using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
   public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int carry = 0;
        int total = 0;
        ListNode* head = new ListNode(-1);
        ListNode* t = head;
        while (l1 != NULL || l2 != NULL || total > 0) {
            if (l1 != NULL) {
                total += l1->val;
                l1 = l1->next;
            }
            if (l2 != NULL) {
                total += l2->val;
                l2 = l2->next;
            }
            if (total >= 10) {
                total -= 10;
                carry = 1;
            }
            t->next = new ListNode(total);
            t = t->next;

            // 忘记设置会死循环
            total = carry;
            carry = 0;
        }
        return head->next;
    }
};
int main() {
    return 0;
}