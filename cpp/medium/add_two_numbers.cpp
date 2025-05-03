// https://leetcode.com/problems/add-two-numbers/

#include <iostream>

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
    static ListNode* addTwoNumbers(ListNode* l1, ListNode* l2);
};

void pushValBack(ListNode **head, ListNode **end, int val) {
    ListNode *l3 = new ListNode(val);
    if (*head == nullptr) {
        *head = l3;
        *end = *head;
    } else {
        (*end)->next = l3;
        *end = (*end)->next;
    }
}

ListNode* Solution::addTwoNumbers(ListNode* l1, ListNode* l2)
{
    int additional = 0;
    int sum = 0;
    ListNode *head = nullptr;
    ListNode *end = nullptr;

    while (l1 != nullptr && l2 != nullptr) {
        sum = l1->val + l2->val + additional;
        if (sum >= 10) {
            additional = 1;
            sum -= 10;
        } else {
            additional = 0;
        }

        pushValBack(&head, &end, sum);

        l1 = l1->next;
        l2 = l2->next;
    }

    while (l1 != nullptr) {
        sum = l1->val + additional;
        if (sum >= 10) {
            additional = 1;
            sum -= 10;
        } else {
            additional = 0;
        }

        pushValBack(&head, &end, sum);

        l1 = l1->next;
    }

    while (l2 != nullptr) {
        sum = l2->val + additional;
        if (sum >= 10) {
            additional = 1;
            sum -= 10;
        } else {
            additional = 0;
        }

        pushValBack(&head, &end, sum);

        l2 = l2->next;
    }

    if (additional != 0) {
        pushValBack(&head, &end, additional);
    }

    return head;
}

int main() {
    auto l1_1 = ListNode(2);
    auto l1_2 = ListNode(3, &l1_1);
    auto l1_3 = ListNode(4, &l1_2);

    auto l2_1 = ListNode(5);
    auto l2_2 = ListNode(6, &l2_1);
    auto l2_3 = ListNode(7, &l2_2);
    auto l2_4 = ListNode(7, &l2_3);

    // 234 + 567 = 801 
    auto l3 = Solution::addTwoNumbers(&l2_3, &l1_3);
    auto l4 = l3;

    while (l4 != nullptr){
        std::cout << l4->val;
        l4 = l4->next;
        delete l3;
        l3 = l4;
    }

    std::cout << std::endl;

    // 234 + 5677 = 5911 
    l3 = Solution::addTwoNumbers(&l2_4, &l1_3);
    l4 = l3;

    while (l4 != nullptr){
        std::cout << l4->val;
        l4 = l4->next;
        delete l3;
        l3 = l4;
    }

    std::cout << std::endl;

    return 0;
}