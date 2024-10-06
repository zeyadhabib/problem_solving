use crate::Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let words2 = sentence2.split_whitespace().collect::<Vec<_>>();

        let mut st1 = 0;
        let mut st2 = 0;
        let mut fn1 = words1.len() as i32 - 1;
        let mut fn2 = words2.len() as i32 - 1;

        while st1 < words1.len() && st2 < words2.len() && words1[st1] == words2[st2] {
            st1 += 1;
            st2 += 1;
        }

        while fn1 >= 0 && fn2 >= 0 && words1[fn1 as usize] == words2[fn2 as usize] {
            fn1 -= 1;
            fn2 -= 1;
        }

        fn2 < st2 as i32 || fn1 < st1 as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn are_sentences_similar() {
        let str1 = "CwFfRo H regR".to_owned();
        let str2 = "CwFfRo regR".to_owned();

        let res = Solution::are_sentences_similar(str1, str2);
        assert!(res);

        let str1 = "c h p Ny".to_owned();
        let str2 = "c BDQ r h p Ny".to_owned();

        let res = Solution::are_sentences_similar(str1, str2);
        assert!(res);

        let str1 = "xD iP tqchblXgqvNVdi".to_owned();
        let str2 = "FmtdCzv Gp YZf UYJ xD iP tqchblXgqvNVdi".to_owned();

        let res = Solution::are_sentences_similar(str1, str2);
        assert!(res);
    }
}
