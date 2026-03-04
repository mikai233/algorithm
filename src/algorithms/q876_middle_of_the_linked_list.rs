use crate::common::{list_node::ListNode, solution::Solution};

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();
        while let Some(f) = fast {
            fast = f.next.as_ref();
            if let Some(f) = fast {
                fast = f.next.as_ref();
                slow = slow.and_then(|n| n.next.as_ref());
            }
        }
        slow.cloned()
    }
}
