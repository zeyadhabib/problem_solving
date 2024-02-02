use crate::Solution;

use std::{collections::VecDeque, vec};

// 739. Daily Temperatures
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut st = VecDeque::new();
        let mut ans = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while let Some(top) = st.back() {
                if temperatures[*top] >= temperatures[i] {
                    break;
                }
                ans[*top] = (i - top) as i32;
                st.pop_back();
            }
            st.push_back(i);
        }
        return ans;
    }
}