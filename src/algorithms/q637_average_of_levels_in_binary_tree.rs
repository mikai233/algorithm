use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut ans = vec![];
        let mut queue = std::collections::VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.clone());
        }
        while !queue.is_empty() {
            let level = queue.len();
            let mut sum = 0i64;
            for _ in 0..level {
                let n = queue.pop_front().unwrap();
                let n = n.borrow();
                sum += n.val as i64;
                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                }
            }
            ans.push(sum as f64 / level as f64);
        }
        ans
    }
}
