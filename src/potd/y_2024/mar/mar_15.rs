use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut product = 1;
        let mut ans = vec![1; n];

        for i in 1..n {
            product *= nums[i - 1];
            ans[i] *= product;
        }

        product = 1;
        for i in (0..(n - 1)).rev() {
            product *= nums[i + 1];
            ans[i] *= product;
        }

        ans
    }
}