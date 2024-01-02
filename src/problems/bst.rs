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
struct Solution {}

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                None => None,
                Some(n) => {
                    let v = n.borrow();
                    if v.val == val {
                        Some(Rc::clone(n))
                    } else if v.val > val {
                        dfs(&v.left, val)
                    } else {
                        dfs(&v.right, val)
                    }
                }
            }
        }
        dfs(&root, val)
    }
}
