use crate::Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        (1..nums.len()).fold(0, |mut result, idx| {
            if nums[idx] <= nums[idx - 1] {
                result += nums[idx - 1] - nums[idx] + 1;
                nums[idx] += nums[idx - 1] - nums[idx] + 1;
            }

            result
        })
    }
}
