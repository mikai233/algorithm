use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        let mut curr = root;
        while let Some(node) = curr {
            let n = node.borrow();
            if p_val < n.val && q_val < n.val {
                curr = n.left.clone();
            } else if p_val > n.val && q_val > n.val {
                curr = n.right.clone();
            } else {
                return Some(node.clone());
            }
        }
        None
    }
}
