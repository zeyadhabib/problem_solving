use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut negatives, mut positives) = nums.iter().fold(
            (VecDeque::new(), VecDeque::new()),
            |(mut negatives, mut positives), num| {
                if *num < 0 {
                    negatives.push_back((*num).abs());
                } else {
                    positives.push_front(*num);
                }

                (negatives, positives)
            },
        );

        let mut ans = Vec::new();
        while !negatives.is_empty() && !positives.is_empty() {
            if negatives.back().unwrap() < positives.back().unwrap() {
                ans.push(negatives.back().unwrap() * negatives.back().unwrap());
                negatives.pop_back();
            } else {
                ans.push(positives.back().unwrap() * positives.back().unwrap());
                positives.pop_back();
            }
        }

        while !negatives.is_empty() {
            ans.push(negatives.back().unwrap() * negatives.back().unwrap());
            negatives.pop_back();
        }

        while !positives.is_empty() {
            ans.push(positives.back().unwrap() * positives.back().unwrap());
            positives.pop_back();
        }

        ans
    }
}
