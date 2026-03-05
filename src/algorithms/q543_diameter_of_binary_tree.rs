use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = 0;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let n = n.borrow_mut();
                    let lh = dfs(n.left.clone(), best);
                    let rh = dfs(n.right.clone(), best);
                    *best = (*best).max(lh + rh);
                    1 + lh.max(rh)
                }
                None => 0,
            }
        }
        dfs(root, &mut best);
        best
    }
}
