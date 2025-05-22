//https://leetcode.com/problems/invert-binary-tree/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let mut value = node.borrow_mut();
                Solution::invert_tree(value.left.clone());
                Solution::invert_tree(value.right.clone());
                match (&value.left, &value.right) {
                    (None, None) => {}
                    (None, Some(_)) => {
                        value.left = value.right.clone();
                        value.right = None;
                    }
                    (Some(_), None) => {
                        value.right = value.left.clone();
                        value.left = None;
                    }
                    (Some(left), Some(right)) => {
                        left.swap(right);
                    }
                };
                drop(value);
                Some(node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::invert_tree(None), None);
    }
}
