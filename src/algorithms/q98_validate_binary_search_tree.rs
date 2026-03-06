use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev: Option<i32> = None;
        let mut st = vec![];
        let mut curr = root;
        while curr.is_some() || !st.is_empty() {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                st.push(node);
            }
            let node = st.pop().unwrap();
            let node = node.borrow();
            if let Some(prev) = prev {
                if prev >= node.val {
                    return false;
                }
            }
            prev = Some(node.val);
            curr = node.right.clone();
        }
        true
    }
}
