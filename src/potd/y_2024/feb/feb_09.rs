use crate::Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut len = 1;
        let mut len_i = 0;
        let mut mem = vec![(1, None); n + 1];
        
        for i in 0..n {
            for j in (0..i).rev() {
                let (j_len, _j_prv) = mem[j];
                let (i_len, i_prv) = &mut mem[i];

                if nums[i] % nums[j] == 0 && *i_len < j_len + 1 {
                    *i_len = j_len + 1;
                    *i_prv = Some(j);
                }

                if len < *i_len {
                    len_i = i;
                    len = *i_len;
                }
            }
        }

        let mut ans = vec![nums[len_i]; len];
        
        while let Some(prev) = mem[len_i].1 {
            ans.push(nums[len_i]);
            len_i = prev;
        }

        ans
    }
}