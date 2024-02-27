use crate::Solution;

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
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
      right: None
    }
  }
}

impl Solution {

    pub fn diameter_of_binary_tree_slv(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let depth_left = Solution::diameter_of_binary_tree_slv(root.borrow().left.clone(), ans);
        let depth_right = Solution::diameter_of_binary_tree_slv(root.borrow().right.clone(), ans);

        *ans = (*ans).max(depth_left + depth_right + 1);
        
        depth_left.max(depth_right) + 1
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Solution::diameter_of_binary_tree_slv(root, &mut ans);
        ans - 1
    }
}