use std::cell::RefCell;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::tree_node::TreeNode;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = std::collections::VecDeque::new();
        match root {
            Some(node) => {
                queue.push_back(node);
            }
            None => {
                return vec![];
            }
        }
        let mut result = vec![];
        while !queue.is_empty() {
            let n = queue.len();
            let mut level = vec![];
            for _ in 0..n {
                let n = queue.pop_front().unwrap();
                let n = n.borrow();
                level.push(n.val);
                if let Some(left) = &n.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &n.right {
                    queue.push_back(right.clone());
                }
            }
            result.push(level);
        }
        result
    }
}
