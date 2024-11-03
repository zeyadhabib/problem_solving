use crate::Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_whitespace().collect::<Vec<_>>();
        (0..words.len()).all(|idx| {
            if let (Some(char1), Some(char2)) = (
                words[idx].chars().next(),
                words[(idx + words.len() - 1) % words.len()].chars().last(),
            ) {
                return char1 == char2;
            }

            false
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn is_circular_sentence() {
        let sentence = String::from("leetcode exercises sound delightful");
        let res = Solution::is_circular_sentence(sentence);
        assert!(res);
    }
}
