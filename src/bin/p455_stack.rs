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
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        while let Some(mut node) = l1 {
            let next = node.next.take();
            stack1.push(node);
            l1 = next;
        }
        while let Some(mut node) = l2 {
            let next = node.next.take();
            stack2.push(node);
            l2 = next;
        }
        let mut new_head: Option<Box<ListNode>> = None;
        let mut carry = 0;
        loop {
            let val = match (stack1.pop(), stack2.pop()) {
                (Some(n1), Some(n2)) => n1.val + n2.val + carry,
                (None, Some(n2)) => n2.val + carry,
                (Some(n1), None) => n1.val + carry,
                (None, None) => break,
            };
            let mut node = Box::new(ListNode::new(val % 10));
            carry = val / 10;
            node.next = new_head;
            new_head = Some(node);
        }
        if carry > 0 {
            let mut node = Box::new(ListNode::new(carry));
            node.next = new_head;
            new_head = Some(node);
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
