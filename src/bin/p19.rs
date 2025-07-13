// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut fast = dummy.clone();
        let mut slow = &mut dummy;
        for _ in 0..n {
            if let Some(next) = fast.next.take() {
                fast = next;
            }
        }
        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        slow.next = slow.next.as_mut().unwrap().next.take();
        dummy.next
    }
}

fn vec_to_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for val in vec {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}


fn main() {
    println!("{:?}", Solution::remove_nth_from_end(vec_to_node(vec![1]), 1));
    println!("{:?}", Solution::remove_nth_from_end(vec_to_node(vec![1, 2, 3, 4, 5]), 2));
    println!("{:?}", Solution::remove_nth_from_end(vec_to_node(vec![1, 2]), 1));
}