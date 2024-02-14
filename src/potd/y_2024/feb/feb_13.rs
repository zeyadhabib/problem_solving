use crate::Solution;

// 2108. Find First Palindromic String in the Array
// TAGS: strings, palindromes, two-pointers
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        
        for word in words {
            let mut left = 0;
            let mut right = word.len() - 1;

            while left < right && word.as_bytes()[left] == word.as_bytes()[right] {
                left += 1;
                right -= 1;
            }

            if left >= right {
                return word;
            }
        }

        return "".to_string();
    }
}