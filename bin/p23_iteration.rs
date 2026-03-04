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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut interval = 1;
        // [n,n,n,n,n,n,n,n]
        // [n, ,n, ,n, ,n, ]
        // [n, , , ,n, , , ]
        // [n, , , , , , , ]
        while interval < lists.len() {
            for i in (0..lists.len() - interval).step_by(interval * 2) {
                lists[i] = Self::merge_two_lists(lists[i].take(), lists[i + interval].take());
            }
            interval *= 2;
        }

        lists[0].take()
    }

    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        tail.next = Some(n1);
                        l1 = tail.next.as_mut().unwrap().next.take();
                        l2 = Some(n2);
                    } else {
                        tail.next = Some(n2);
                        l2 = tail.next.as_mut().unwrap().next.take();
                        l1 = Some(n1);
                    }
                    tail = tail.next.as_mut().unwrap();
                }
                (Some(n), None) | (None, Some(n)) => {
                    tail.next = Some(n);
                    break;
                }
                (None, None) => break,
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
    let list1 = vec_to_node(vec![1, 4, 5]);
    let list2 = vec_to_node(vec![1, 3, 4]);
    let list3 = vec_to_node(vec![2, 6]);
    println!("{:?}", Solution::merge_k_lists(vec![list1, list2, list3]));
}
