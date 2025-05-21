# https://leetcode.com/problems/merge-k-sorted-lists/description/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

import heapq
from typing import List, Optional

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:

        heap = []
        counter = 0;

        while counter < len(lists):
            for i in range(0, len(lists)):
                if lists[i] is None:
                    counter = counter + 1
                    continue;
                else:
                    counter = 0

                heapq.heappush(heap, lists[i].val)
                lists[i] = lists[i].next

        if len(heap) == 0:
            return None

        heapq.heapify(heap)
        
        result = ListNode(heapq.heappop(heap))
        pointer = result;
        for i in range(0, len(heap)):
            pointer.next = ListNode(heapq.heappop(heap))
            pointer = pointer.next

        return result

if __name__ == '__main__':
    s = Solution()

    n1_1 = ListNode(3)
    n1_2 = ListNode(2, n1_1)
    n1_3 = ListNode(0, n1_2)


    n2_1 = ListNode(4)
    n2_2 = ListNode(1, n2_1)
    n2_3 = ListNode(1, n2_2)
    n2_4 = ListNode(0, n2_3)

    result = s.mergeKLists([])
    assert(result is None)

    result = s.mergeKLists([None])
    assert(result is None)

    s.mergeKLists([n1_3])

    result = s.mergeKLists([n1_3, n2_4])
    # p = result
    # while p is not None:
    #     print(p.val)
    #     p = p.next


    n1_1 = ListNode(5)
    n1_2 = ListNode(4, n1_1)
    n1_3 = ListNode(1, n1_2)


    n2_1 = ListNode(4)
    n2_2 = ListNode(3, n2_1)
    n2_3 = ListNode(1, n2_2)

    n3_1 = ListNode(6)
    n3_2 = ListNode(2, n3_1)

    result = s.mergeKLists([n1_3, n2_3, n3_2])
    p = result
    while p is not None:
        print(p.val)
        p = p.next


