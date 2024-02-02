use crate::Solution;

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

use std::rc::Rc;
use std::cell::RefCell;

// 1457. Pseudo-Palindromic Paths in a Binary Tree
impl Solution {

    pub fn is_pseudo_palindromic (freq: Rc<RefCell<Vec<i32>>>) -> i32 {
        (
            freq.borrow()
                .iter()
                .fold(0, |acc, x| acc + (x % 2)) <= 1
        ) as i32
    }

    pub fn slv (root: Option<Rc<RefCell<TreeNode>>>, freq: Rc<RefCell<Vec<i32>>>) -> i32 {
        match root {
            None => 0 as i32,
            Some (root) => {
                let root = root.borrow();
                {
                    let mut freq = freq.borrow_mut();
                    freq[root.val as usize] += 1;
                }
                let mut ans = 0;
                if root.left.is_none() && root.right.is_none() {
                    ans = Solution::is_pseudo_palindromic(freq.clone());
                } else {
                    ans += Solution::slv(root.left.clone(), freq.clone());
                    ans += Solution::slv(root.right.clone(), freq.clone());
                }

                let mut freq = freq.borrow_mut();
                freq[root.val as usize] -= 1;

                ans
            }
        }
    }

    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::slv(root, Rc::new(RefCell::new(vec![0; 10])))
    }
}

