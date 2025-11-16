use crate::Solution;

use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let dp = Rc::new(RefCell::new(vec![1; nums.len()]));
        (0..nums.len())
            .fold((1, dp), |(mut max, dp), idx| {
                for i in 0..idx {
                    if nums[idx] > nums[i] {
                        let mut dp = dp.borrow_mut();
                        dp[idx] = dp[idx].max(dp[i] + 1);
                        max = max.max(dp[idx]);
                    }
                }
                (max, dp)
            })
            .0
    }
}
