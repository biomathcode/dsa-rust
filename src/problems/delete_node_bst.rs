// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let node_val = node.borrow().val;

                if key < node_val {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Solution::delete_node(left, key);
                } else if key > node_val {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Solution::delete_node(right, key);
                } else {
                    if node.borrow().left.is_none() {
                        return node.borrow().right.clone();
                    } else if node.borrow().right.is_none() {
                        return node.borrow().left.clone();
                    }

                    // Node has both left and right children
                    let successor = Solution::get_successor(node.borrow().right.clone());
                    node.borrow_mut().val = successor.borrow().val;
                    let new_right = Solution::delete_node(
                        node.borrow().right.clone(),
                        successor.borrow().val
                    );
                    node.borrow_mut().right = new_right;
                }
                Some(node)
            }
        }
    }

    fn get_successor(root: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        let mut current = root;
        while let Some(node) = current.clone() {
            current = node.borrow().left.clone();
        }
        current.unwrap()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn should_delete_node_with_given_key() {
        //
    }
}
