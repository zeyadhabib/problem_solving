use crate::Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let ans = f32::sqrt(n as f32 * (n + 1) as f32 * 0.5) as i32;
        match ans * ans == n * (n + 1) / 2 {
            true => ans,
            false => -1,
        }
    }
}
