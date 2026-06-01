use crate::Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort();
        cost.iter().rev().enumerate().fold(0, |total, (cnt, num)| {
            if (cnt + 1) % 3 != 0 {
                total + num
            } else {
                total
            }
        })
    }
}
