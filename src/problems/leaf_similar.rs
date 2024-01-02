// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;

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
struct Solution {}

impl Solution {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(n) = node {
            let n_borrow = n.borrow();

            if n_borrow.left.is_none() && n_borrow.right.is_none() {
                result.push(n_borrow.val);
            } else {
                result.extend(Solution::dfs(n_borrow.left.clone()));
                result.extend(Solution::dfs(n_borrow.right.clone()));
            }
        }
        result
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        let mut tree_one = Solution::dfs(root1);
        let mut tree_two = Solution::dfs(root2);

        tree_one == tree_two
    }
}
