use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, mut ans: &mut Option<i32>) {
            match (node, &mut ans) {
                (Some(node), _) => {
                    let node = node.borrow();
                    dfs(node.left.clone(), k, ans);
                    *k -= 1;
                    if *k == 0 {
                        *ans = Some(node.val);
                    }
                    dfs(node.right.clone(), k, ans);
                }
                _ => {}
            }
        }
        let mut ans = None;
        dfs(root, &mut k, &mut ans);
        ans.unwrap_or_default()
    }
}
