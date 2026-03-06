use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        nums = nums
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<i32>>();
        nums.sort();

        nums.iter()
            .enumerate()
            .fold((0, 0), |(acc, ans), (idx, num)| {
                if idx > 0 && num - nums[idx - 1] == 1 {
                    (acc + 1, ans.max(acc + 1))
                } else {
                    (1, ans.max(1))
                }
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
        assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
    }
}
