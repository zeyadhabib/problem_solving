use crate::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let idx = word.find(ch);
        match idx {
            Some(idx) => {
                let (left, right) = ((&word[0..=idx]).to_owned(), &word[(idx + 1)..]);
                left.chars().rev().collect::<String>() + right
            }
            None => word,
        }
    }
}
