use crate::Solution;

use std::collections::HashSet;

// 349. Intersection of Two Arrays
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let seen = nums1.iter().fold(HashSet::new(), |mut seen, num| {
            seen.insert(*num);
            seen
        });

        nums2.iter().fold(HashSet::new(), |mut seen2, num|{
            if seen.contains(num) {
                seen2.insert(*num);
            }
            seen2
        }).iter().map(|num| *num).collect::<Vec<i32>>()
    }
}

// 918. Maximum Sum Circular Subarray
// TAGS: Kadane's algorithm
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (all_negative, max, acc) = nums.iter().fold((true, i32::MIN, 0), |(mut all_negative, mut max, mut acc), num| {
            all_negative = all_negative && (*num < 0);
            max = max.max(*num);
            acc += *num;
            (all_negative, max, acc)
        });

        if all_negative {
            return max;
        }
        
        let mut sum_pos = 0;
        let mut sum_neg = 0;
        let mut ans = i32::MIN;

        for num in nums {
            sum_pos += num;
            sum_neg += num;

            ans = ans.max(sum_pos);
            ans = ans.max(acc - sum_neg);

            sum_pos = sum_pos.max(0);
            sum_neg = sum_neg.min(0);
        }
        
        ans
    }
}