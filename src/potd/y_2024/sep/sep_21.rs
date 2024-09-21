use crate::Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut arr = (1..=n).collect::<Vec<_>>();
        arr.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        arr
    }
}
