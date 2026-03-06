use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut st = vec![];
        let mut curr = root;
        let mut min_diff = u32::MAX;
        let mut prev: Option<i32> = None;
        while curr.is_some() || !st.is_empty() {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                st.push(node);
            }
            let node = st.pop().unwrap();
            if let Some(p) = prev {
                min_diff = min_diff.min(p.abs_diff(node.borrow().val));
            }
            prev = Some(node.borrow().val);
            curr = node.borrow().right.clone();
        }
        min_diff as i32
    }
}
