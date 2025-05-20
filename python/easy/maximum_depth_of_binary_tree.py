# https://leetcode.com/problems/maximum-depth-of-binary-tree/

from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        if root.left is None and root.right is None:
            return 1

        return max(self.maxDepth(root.left), self.maxDepth(root.right)) + 1


if __name__ == '__main__':
    s = Solution()
    s.maxDepth(None)