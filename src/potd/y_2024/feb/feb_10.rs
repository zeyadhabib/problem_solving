use crate::Solution;

// 647. Palindromic Substrings
// TAGS: Palindromes, strings
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();

        let (evens, odds) = (0..n).fold((0, 0), |(mut evens, mut odds), i| {
            let mut left = i as i32;
            let mut right = i;

            while left >= 0 && right < n && s.as_bytes()[left as usize] == s.as_bytes()[right] {
                odds += 1;
                left -= 1;
                right += 1;
            }

            left = i as i32;
            right = i + 1;

            while left >= 0 && right < n && s.as_bytes()[left as usize] == s.as_bytes()[right] {
                evens += 1;
                left -= 1;
                right += 1;
            }

            (evens, odds)
        });

        odds + evens
    }
}