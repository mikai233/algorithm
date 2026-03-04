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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut prev = &mut dummy;
        loop {
            match (list1, list2) {
                (Some(mut l1), Some(mut l2)) => {
                    if l1.val <= l2.val {
                        list1 = l1.next.take();
                        list2 = Some(l2);
                        prev.next = Some(l1);
                    } else {
                        list2 = l2.next.take();
                        list1 = Some(l1);
                        prev.next = Some(l2);
                    }
                    prev = prev.next.as_mut().unwrap();
                }
                (Some(l1), None) => {
                    prev.next = Some(l1);
                    break;
                }
                (None, Some(l2)) => {
                    prev.next = Some(l2);
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
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
    let list1 = vec_to_node(vec![1, 2, 4]);
    let list2 = vec_to_node(vec![1, 3, 4]);
    println!("{:?}", Solution::merge_two_lists(list1, list2));
    println!("{:?}", Solution::merge_two_lists(None, None));
    let list2 = vec_to_node(vec![0]);
    println!("{:?}", Solution::merge_two_lists(None, list2));
}
