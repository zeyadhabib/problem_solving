use crate::Solution;

use std::{collections::HashMap, i32};

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, Vec<usize>>::new();
        let ans = nums.iter().enumerate().fold(i32::MAX, |ans, (idx, &num)| {
            let last_3 = map.entry(num).or_default();
            last_3.push(idx);
            if last_3.len() > 3 {
                last_3.remove(0);
            }
            if last_3.len() == 3 {
                let val = last_3[0].abs_diff(last_3[1])
                    + last_3[1].abs_diff(last_3[2])
                    + last_3[2].abs_diff(last_3[0]);
                ans.min(val as i32)
            } else {
                ans
            }
        });

        match ans {
            i32::MAX => -1,
            _ => ans,
        }
    }
}
