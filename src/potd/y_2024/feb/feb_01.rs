use crate::Solution;

// 2966. Divide Array Into Arrays With Max
impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let ans = nums.iter()
            .enumerate()
            .step_by(3)
            .try_fold(Vec::<Vec<i32>>::new(), |mut ans, (i, num)| {
                if nums[i + 2] - num <= k {
                    ans.push(nums[i..=(i + 2)].to_vec());
                } else {
                    return Err("");
                }
                Ok(ans)
            });
        
        ans.unwrap_or(Vec::<Vec<i32>>::new())
    }
}

impl Solution {
    pub fn divide_array_dup(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut nums = nums;
        let mut ans = Vec::<Vec<i32>>::new();
        nums.sort();

        if nums.len() %  3 != 0 {
            return Vec::<Vec<i32>>::new();
        }

        for i in (2..n as usize).step_by(3) {
            if nums[i] - nums[i - 2] <= k {
                ans.push(nums[(i - 2)..=i].to_vec());
            } else {
                return Vec::<Vec<i32>>::new();
            }
        }

        return ans;
    }
}