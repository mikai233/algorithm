use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let mut root = root.borrow_mut();
            let left = Solution::invert_tree(root.left.clone());
            let right = Solution::invert_tree(root.right.clone());
            root.left = right;
            root.right = left;
        }
        root
    }
}
