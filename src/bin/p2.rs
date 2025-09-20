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
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut result = ListNode::new(0);
        let mut current = &mut result;
        let mut carry = 0;
        loop {
            let val = match (l1, l2) {
                (None, Some(n)) | (Some(n), None) => n.val + carry,
                (Some(s1), Some(s2)) => s1.val + s2.val + carry,
                (None, None) => {
                    break;
                }
            };
            let node = ListNode::new(val % 10);
            current.next = Some(Box::new(node));
            carry = val / 10;
            current = current.next.as_mut().unwrap();
            l1 = l1.and_then(|l1| l1.next.as_ref());
            l2 = l2.and_then(|l2| l2.next.as_ref());
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }
        result.next
    }
}

fn vec_to_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;
    for x in vec {
        current.next = Some(Box::new(ListNode::new(x)));
        current = current.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    assert_eq!(
        Solution::add_two_numbers(vec_to_node(vec![2, 4, 3]), vec_to_node(vec![5, 6, 4])),
        vec_to_node(vec![7, 0, 8])
    );
    assert_eq!(
        Solution::add_two_numbers(vec_to_node(vec![0]), vec_to_node(vec![0])),
        vec_to_node(vec![0])
    );
    assert_eq!(
        Solution::add_two_numbers(
            vec_to_node(vec![9, 9, 9, 9, 9, 9, 9]),
            vec_to_node(vec![9, 9, 9, 9])
        ),
        vec_to_node(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}
