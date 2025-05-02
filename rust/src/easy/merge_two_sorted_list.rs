// https://leetcode.com/problems/merge-two-sorted-lists/


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (_, None) => list1,
            (None, _) => list2,
            (Some(mut head1), Some(mut head2)) => {
                if head1.val < head2.val {
                    if head1.next.is_none() {
                        head1.next = list2;
                        list1
                    } else {
                        
                    }
                    while True {
                        match head1.next {
                            None => {
                                head1.next = Some(head2);
                                return list1;
                            },
                            Some(node) if node.val < head2.val => {
                                head1 = head.next.unwrap();
                            },
                            node => {
                                let broken = node;
                                // head1.next = head2
                                // head2 = broken

                            }

                        }
                    }
                } else {

                }

            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
