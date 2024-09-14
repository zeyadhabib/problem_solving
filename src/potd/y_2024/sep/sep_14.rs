use crate::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let &max_num = nums.iter().max().unwrap();
        nums.iter()
            .fold((0, 0), |(len, ans), &curr| {
                if curr == max_num {
                    (len + 1, ans.max(len + 1))
                } else {
                    (0, ans)
                }
            })
            .1
    }
}
