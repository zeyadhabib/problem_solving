use crate::Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut sum = nums.iter().map(|i| *i as i64).sum::<i64>();

        nums.sort_by(|a, b| b.cmp(a));
        for num in nums {
            sum -= num as i64;
            if sum > num as i64 {
                return sum + num as i64;
            }
        }

        -1
    }
}