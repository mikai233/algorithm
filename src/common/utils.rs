use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

/// Quickly construct a Vec<String>.
/// Usage: vec_str!["eat", "tea", "tan"]
#[macro_export]
macro_rules! vec_str {
    ($($x:expr),* $(,)?) => (vec![$($x.to_string()),*]);
}

/// Quickly construct a 2D array (Vec<Vec<T>>).
/// Usage: vec_vec![[1, 2], [3, 4]]
#[macro_export]
macro_rules! vec_vec {
    ($([$($x:expr),* $(,)?]),* $(,)?) => (vec![$(vec![$($x.into()),*]),*]);
}

/// Quickly construct a singly linked list.
/// Usage: linked_list![1, 2, 3, 4, 5]
#[macro_export]
macro_rules! linked_list {
    () => { None };
    ($($e:expr),* $(,)?) => {
        {
            let mut head = None;
            // Collect all elements and insert them in reverse order
            let mut elements = vec![$($e),*];
            while let Some(val) = elements.pop() {
                let mut node = $crate::common::list_node::ListNode::new(val);
                node.next = head;
                head = Some(Box::new(node));
            }
            head
        }
    };
}

/// Helper macro: Converts 'null' to None, and values to Some(val).
#[macro_export]
macro_rules! tree_node_val {
    (null) => {
        None
    };
    ($e:expr) => {
        Some($e)
    };
}

/// Internal function: Builds a binary tree from a level-order traversal array.
pub fn build_tree_from_level_order(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;

    while i < values.len() {
        if let Some(node) = queue.pop_front() {
            // Process the left child
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
                i += 1;
            }
            // Process the right child
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

/// Quickly construct a binary tree, supporting LeetCode's 'null' syntax.
/// Usage: tree![1, null, 2, 3]
#[macro_export]
macro_rules! tree {
    ($($e:tt),* $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($crate::tree_node_val!($e));
            )*
            $crate::common::utils::build_tree_from_level_order(vec)
        }
    };
}
