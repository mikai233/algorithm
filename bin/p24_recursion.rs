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
        head.map(|mut n1| match n1.next {
            None => n1,
            Some(mut n2) => {
                n1.next = Solution::swap_pairs(n2.next);
                n2.next = Some(n1);
                n2
            }
        })
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
