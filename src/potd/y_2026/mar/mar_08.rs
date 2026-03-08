use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums[0].len();
        let mut set = HashSet::new();
        for num in nums {
            let bytes = num.bytes();
            let int_num = bytes.fold(0, |acc, bit| acc * 2 + (bit - '0' as u8) as i32);

            set.insert(int_num);
        }

        for entry in set.iter() {
            let plus_1 = set.get(&(entry + 1));
            let minus_1 = set.get(&(entry - 1));

            if plus_1.is_none() {
                let ans = entry + 1;
                if ans.ilog2() < n as u32 {
                    return format!("{:0>width$b}", ans, width = n);
                }
            }

            if minus_1.is_none() {
                let ans = entry - 1;
                if ans < 0 {
                    continue;
                }

                return format!("{:0>width$b}", ans, width = n);
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_find_different_binary_string() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                "111".to_string(),
                "011".to_string(),
                "001".to_string()
            ]),
            "110"
        );
        assert_eq!(
            Solution::find_different_binary_string(vec!["1".to_string()]),
            "0"
        );
    }
}
