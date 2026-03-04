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
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut tail = &mut dummy;

        while let Some(mut n1) = tail.next.take() {
            if let Some(mut n2) = n1.next.take() {
                n1.next = n2.next.take();
                n2.next = Some(n1);
                tail.next = Some(n2);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                tail.next = Some(n1);
                break;
            }
        }
        // while tail.next.is_some() && tail.next.as_ref().unwrap().next.is_some() {
        //     let temp = tail.next.as_mut().unwrap().next.as_mut().unwrap().next.take();
        //     let mut n1 = tail.next.take();
        //     let mut n2 = n1.as_mut().unwrap().next.take();
        //     n1.as_mut().unwrap().next = temp;
        //     n2.as_mut().unwrap().next = n1;
        //     tail.next = n2;
        //     tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
        // }
        dummy.next
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
