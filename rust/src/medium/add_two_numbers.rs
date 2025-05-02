// https://leetcode.com/problems/add-two-numbers/description/

// Definition for singly-linked list.
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut additional = None;

    let mut l1_pointer = l1;
    let mut l2_pointer = l2;

    let mut v = Vec::new();

    let mut result = None;

    while l1_pointer.is_some() || l2_pointer.is_some() {
        let mut sum = additional.unwrap_or(0);
        if let Some(l1_node) = l1_pointer {
            sum += l1_node.val;
            l1_pointer = l1_node.next;
        }
        if let Some(l2_node) = l2_pointer {
            sum += l2_node.val;
            l2_pointer = l2_node.next;
        }

        if sum >= 10 {
            additional = Some(sum / 10);
            sum %= 10;
        } else {
            additional = None;
        }

        v.push(sum);
    }

    if let Some(val) = additional {
        v.push(val);
    }

    for item in v.iter().rev() {
        let list_node = ListNode {
            val: *item,
            next: result,
        };

        result = Some(Box::new(list_node));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // `$ cargo test -- add_two_numbers --nocapture`

        let l1_1 = ListNode::new(2);

        let mut l1_2 = ListNode::new(3);
        l1_2.next = Some(Box::new(l1_1));

        let l1_3 = ListNode {
            val: 4,
            next: Some(Box::new(l1_2)),
        };

        let l2_1 = ListNode::new(9);

        let mut l2_2 = ListNode::new(3);
        l2_2.next = Some(Box::new(l2_1));

        let l2_3 = ListNode {
            val: 8,
            next: Some(Box::new(l2_2)),
        };

        // 234 + 938 = 1172
        println!(
            "{:?}",
            add_two_numbers(Some(Box::new(l1_3.clone())), Some(Box::new(l2_3.clone())))
        );

        let l2_4 = ListNode {
            val: 1,
            next: Some(Box::new(l2_3)),
        };

        // 234 + 9381 = 9615
        println!(
            "{:?}",
            add_two_numbers(Some(Box::new(l1_3)), Some(Box::new(l2_4)))
        );
    }
}
