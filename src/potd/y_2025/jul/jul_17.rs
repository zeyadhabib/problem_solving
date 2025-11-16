use crate::Solution;

use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .fold(
                (
                    0,
                    Rc::new(RefCell::new(vec![vec![1; k as usize]; nums.len()])),
                ),
                |(mut answer, dp), (idx_i, num_i)| {
                    {
                        let mut dp = dp.borrow_mut();
                        nums.iter()
                            .enumerate()
                            .take(idx_i)
                            .for_each(|(idx_j, num_j)| {
                                let remainder = ((num_i + num_j) % k) as usize;
                                dp[idx_i][remainder] =
                                    dp[idx_i][remainder].max(dp[idx_j][remainder] + 1);
                                answer = answer.max(dp[idx_i][remainder])
                            });
                    }

                    (answer, dp)
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_length() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
    }
}
