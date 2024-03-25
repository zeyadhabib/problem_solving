use crate::Solution;

impl Solution {
    pub fn find_duplicate_old(mut nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .try_fold(-1, |ans, idx| {
                let num = nums[idx].abs();
                if nums[num as usize] < 0 {
                    return Err(num);
                }

                nums[num as usize] *= -1;
                Ok(ans)
            })
            .unwrap_err()
    }
}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut ans = 0 as i32;
        let mut st = 0 as i32;
        let mut fin = (nums.len() - 1) as i32;

        while st <= fin {
            let mid = (fin - st) / 2 + st;
            let count = nums.iter().fold(0, |acc, num| acc + (*num <= mid) as i32);
            if count <= mid {
                st = mid + 1;
            } else {
                ans = mid;
                fin = mid - 1;
            }
        }

        ans
    }
}
