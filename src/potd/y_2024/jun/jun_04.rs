use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut frequency = vec![0; 256];
        s.chars().for_each(|char_i| {
            frequency[char_i as usize] += 1;
        });

        frequency
            .iter()
            .fold((false, 0), |(odd, length), curr| {
                let curr_odd = (curr % 2) == 1;
                let decrement = curr_odd as i32;
                (
                    (odd | curr_odd),
                    (length + curr - decrement + (curr_odd && !odd) as i32),
                )
            })
            .1
    }
}
