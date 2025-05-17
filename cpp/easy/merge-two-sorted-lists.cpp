// https://leetcode.com/problems/merge-two-sorted-lists/


// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
    public:
        ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
            if (list1 == nullptr && list2 == nullptr)
                return nullptr;
            
            if (list1 == nullptr)
                return list2;
            
            if (list2 == nullptr)
                return list1;

            auto p1 = list1;
            auto p2 = list2;

            ListNode* new_list = nullptr;
            auto p3 = &new_list;

            while (p1 != nullptr || p2 != nullptr) {
                if (p1 ==  nullptr || (p2 != nullptr && p1->val > p2->val)){
                    *p3 = new ListNode(p2->val);
                    p2 = p2->next;
                    p3 = &(*p3)->next;
                } else if (p2 == nullptr || ( p1 != nullptr && p2->val > p1->val)) {
                    *p3 = new ListNode(p1->val);
                    p1 = p1->next;
                    p3 = &(*p3)->next;
                } else {
                    *p3 = new ListNode(p1->val);
                    p3 = &(*p3)->next;
                    *p3 = new ListNode(p2->val);
                    p3 = &(*p3)->next;
                    p1 = p1->next;
                    p2 = p2->next;
                }
            }

            return new_list;
        }
    };

int main() {
    return 0;
}