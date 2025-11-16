use std::i32;

use crate::Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let st = match s.find('1') {
            Some(idx) => idx,
            None => s.len(),
        };
        s.char_indices()
            .skip(st)
            .fold((st, 0 as i64), |(mut st, mut ans), (fin, ch)| {
                if ch == '0' || fin == s.len() - 1 {
                    let n = (ch == '1') as i64 + (fin - st) as i64;
                    ans += (n * (n + 1) / 2).rem_euclid(1e9 as i64 + 7);
                    st = fin + 1;
                }

                (st, ans)
            })
            .1 as i32
    }
}

impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let (a, b, c) = nums.iter().fold(
            (i32::MIN, i32::MIN, i32::MAX),
            |(mut max, mut max2, min), &elem| {
                if elem > max {
                    max2 = max;
                    max = elem;
                } else if elem > max2 {
                    max2 = elem;
                }

                (max, max2, min.min(elem))
            },
        );

        a + b - c
    }
}
