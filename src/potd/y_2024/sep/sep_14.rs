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

impl Solution {
    pub fn string_hash_2(s: String, k: i32) -> String {
        (0..(s.len() / k as usize))
            .map(|idx| {
                ((0..k)
                    .zip(s.as_bytes().iter().skip(idx * k as usize))
                    .fold(0 as u8, |sum, (_i, &ch)| {
                        (sum as u8 % 26 + (ch - 'a' as u8) % 26) % 26 as u8
                    })
                    + 'a' as u8) as char
            })
            .collect::<String>()
    }
}

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        s.as_bytes()
            .chunks(k as usize)
            .map(|chunck| {
                (chunck
                    .iter()
                    .fold(0 as u8, |sum, &ch| (sum % 26 + (ch - 'a' as u8) % 26) % 26)
                    + 'a' as u8) as char
            })
            .collect::<String>()
    }
}
