use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn lowest_common_ancestor_236(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: i32,
            q: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    if n.val == p || n.val == q {
                        return Some(node.clone());
                    }
                    let left = dfs(n.left.clone(), p, q);
                    let right = dfs(n.right.clone(), p, q);
                    match (left, right) {
                        (None, None) => None,
                        (None, Some(r)) => Some(r.clone()),
                        (Some(l), None) => Some(l.clone()),
                        (Some(_), Some(_)) => Some(node.clone()),
                    }
                }
                None => None,
            }
        }
        dfs(root, p_val, q_val)
    }
}
