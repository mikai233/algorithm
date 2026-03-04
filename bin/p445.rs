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

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        l1 = Self::reverse_list(l1);
        l2 = Self::reverse_list(l2);
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        loop {
            let val = match (&mut l1, &mut l2) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => break,
            };
            curr.next = Some(Box::new(ListNode::new(val % 10)));
            curr = curr.next.as_mut().unwrap();
            carry = val / 10;
            l1 = l1.and_then(|l1| l1.next);
            l2 = l2.and_then(|l2| l2.next);
        }
        if carry > 0 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }
        Self::reverse_list(dummy.next)
    }

    fn reverse_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        while let Some(mut node) = list {
            let temp = node.next;
            node.next = new_head;
            new_head = Some(node);
            if temp.is_some() {
                list = temp
            } else {
                break;
            }
        }
        new_head
    }
}

fn vec_to_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut root = Box::new(ListNode::new(0));
    let mut current = &mut root;
    for x in vec {
        current.next = Some(Box::new(ListNode::new(x)));
        current = current.next.as_mut().unwrap();
    }
    root.next
}

fn main() {
    println!(
        "{:?}",
        Solution::add_two_numbers(vec_to_node(vec![7, 2, 4, 3]), vec_to_node(vec![5, 6, 4]))
    );
}
