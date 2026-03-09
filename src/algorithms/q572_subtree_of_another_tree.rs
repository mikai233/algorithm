use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_same_tree(
            a: Option<Rc<RefCell<TreeNode>>>,
            b: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (a, b) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val
                        && is_same_tree(a.left.clone(), b.left.clone())
                        && is_same_tree(a.right.clone(), b.right.clone())
                }
            }
        }
        match (root, sub_root) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (Some(root), Some(sub)) => {
                if is_same_tree(Some(root.clone()), Some(sub.clone())) {
                    return true;
                }
                let r = root.borrow();
                Solution::is_subtree(r.left.clone(), Some(sub.clone()))
                    || Solution::is_subtree(r.right.clone(), Some(sub))
            }
        }
    }
}
