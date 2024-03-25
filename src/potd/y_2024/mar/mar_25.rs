use crate::Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).fold(Vec::new(), |mut ans, idx| {
            let num = nums[idx].abs() - 1;
            if nums[num as usize] < 0 {
                ans.push(num + 1);
                return ans;
            }

            nums[num as usize] *= -1;
            ans
        })
    }
}
