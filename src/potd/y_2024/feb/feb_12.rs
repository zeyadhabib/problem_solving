use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, -1), |(mut votes, mut candidate), n|{
            if votes == 0 {
                candidate = *n;
            }
            votes += match candidate == *n {
                true => 1,
                false => -1
            };
            (votes, candidate)
        }).1
    }
}