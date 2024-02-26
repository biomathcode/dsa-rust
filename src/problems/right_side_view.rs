use std::collections::VecDeque;
// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();

    if root == None {
        return vec![];
    }

    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while !queue.is_empty() {
            let level_size = queue.len();

            for i in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    if i == level_size - 1 {
                        result.push(node.val);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }

                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                }
            }
        }
    }

    result
}
