// https://leetcode.com/problems/middle-of-the-linked-list/description/

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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut end = head.clone();
    let mut middle = head;

    while end.is_some() {
        
        end = end.unwrap().next;

        if let Some(val) = end {
            middle = middle.unwrap().next;
            end = val.next;
        }
    }

    return middle;
}

fn main() {
    let l = Box::new(ListNode::new(123));
    let list = ListNode {
        val: 234,
        next: Some(l),
    };

    println!("{:?}", middle_node(Some(Box::new(list))));
}
