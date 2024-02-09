use crate::Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut max_len = 1;
        let mut max_len_i = 0;
        let mut mem = vec![(1, None); n + 1];
        
        for i in 0..n {
            for j in (0..i).rev() {
                let (j_len, _j_prv) = mem[j];
                let (i_len, i_prv) = &mut mem[i];

                if nums[i] % nums[j] == 0 && *i_len < j_len + 1 {
                    *i_len = j_len + 1;
                    *i_prv = Some(j);
                }

                if max_len < *i_len {
                    max_len_i = i;
                    max_len = *i_len;
                }
            }
        }

        let mut ans = vec![nums[max_len_i]];
        
        while let Some(prev) = mem[max_len_i].1 {
            ans.push(nums[prev]);
            max_len_i = prev;
        }

        ans
    }
}