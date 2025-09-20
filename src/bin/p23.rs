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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        Self::merge(&lists, 0, lists.len() - 1)
    }

    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }

    fn merge(
        lists: &Vec<Option<Box<ListNode>>>,
        left: usize,
        right: usize,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return lists[left].clone();
        }
        let mid = left + (right - left) / 2;
        let l1 = Self::merge(lists, left, mid);
        let r1 = Self::merge(lists, mid + 1, right);
        Self::merge_two_lists(l1, r1)
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
