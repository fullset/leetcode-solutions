// https://leetcode.com/problems/reverse-linked-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(_) => {
                let mut item = head;
                let mut prev_item = None;

                while let Some(mut wrapped) = item {
                    item = wrapped.next;
                    wrapped.next = prev_item;
                    prev_item = Some(wrapped);
                }

                prev_item
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // `$ cargo test -- reverse_linked_list --nocapture`
        let l1_1 = ListNode::new(2);

        let mut l1_2 = ListNode::new(3);
        l1_2.next = Some(Box::new(l1_1));

        let l1_3 = ListNode {
            val: 4,
            next: Some(Box::new(l1_2)),
        };

        println!("{:?}", l1_3);
        println!("{:?}", Solution::reverse_list(Some(Box::new(l1_3))));
    }
}
