use crate::common::{list_node::ListNode, solution::Solution};

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut len = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }
        if len <= 1 {
            return true;
        }
        let first_half_last = len / 2 - 1;
        let mut last = head.as_mut();
        for _ in 0..first_half_last {
            last = last.unwrap().next.as_mut();
        }
        let second_half = if len % 2 == 0 {
            last.unwrap().next.take()
        } else {
            let mut next = last.unwrap().next.take();
            next.as_mut().unwrap().next.take()
        };
        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
            prev
        }
        let mut first_half = head;
        let mut second_half = reverse(second_half);
        loop {
            match (first_half, second_half) {
                (None, None) => break true,
                (None, Some(_)) | (Some(_), None) => break false,
                (Some(l), Some(r)) => {
                    if l.val != r.val {
                        break false;
                    } else {
                        first_half = l.next;
                        second_half = r.next;
                    }
                }
            }
        }
    }
}
