use crate::Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positive = 0;
        let mut negative = 1;
        let mut solution = vec![0; nums.len()];

        for num in nums {
            if num > 0 {
                solution[positive] = num;
                positive += 2;
            } else {
                solution[negative] = num;
                negative += 2;
            }
        }

        solution
    }
}