use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn judge_square_sum_1(c: i32) -> bool {
        let set = (0..=((c as f64).sqrt().floor() as i32))
            .map(|i| i * i)
            .collect::<HashSet<i32>>();

        for a_squared in set.iter() {
            if let Some(_b_squared) = set.get(&(c - *a_squared)) {
                return true;
            }
        }
        
        false
    }
}


impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut left = 0;
        let mut right = (c as f64).sqrt().floor() as i32;

        while left <= right {
            if left * left + right * right == c {
                return true;
            } else if left * left + right * right > c {
                right -= 1;
            } else {
                left += 1;
            }
        }

        false
    }
}
