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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if let Some(mut next_node) = curr {
                curr = next_node.next.take();
                stack.push(next_node);
                stack.push(node);
            } else {
                stack.push(node);
            }
        }

        let mut new_head = None;
        while let Some(mut node) = stack.pop() {
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
    println!("{:?}", Solution::swap_pairs(vec_to_node(vec![1, 2, 3, 4])));
    println!("{:?}", Solution::swap_pairs(vec_to_node(vec![])));
    println!("{:?}", Solution::swap_pairs(vec_to_node(vec![1])));
    println!("{:?}", Solution::swap_pairs(vec_to_node(vec![1, 2, 3])));
}
