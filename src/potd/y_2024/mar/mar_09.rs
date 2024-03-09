use crate::Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;

        while left < nums1.len() && right < nums2.len() {
            if nums1[left] == nums2[right] {
                return nums1[left];
            }

            if nums1[left] < nums2[right] {
                left += 1;
            } else {
                right += 1;
            }
        }

        -1
    }
}