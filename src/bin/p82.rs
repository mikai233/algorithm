struct Solution;

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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;
        while let Some(mut curr) = prev.next.take() {
            let mut duplicated = false;
            while let Some(next) = curr.next.as_ref() {
                if curr.val == next.val {
                    duplicated = true;
                    let next_next = curr.next.as_mut().unwrap().next.take();
                    curr.next = next_next;
                } else {
                    break;
                }
            }
            if duplicated {
                prev.next = curr.next;
            } else {
                prev.next = Some(curr);
                prev = prev.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

fn main() {}
