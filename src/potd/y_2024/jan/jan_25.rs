use crate::Solution;

// 1143. Longest Common Subsequence
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut mem = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for i in 1..=text1.len() {
            for j in 1..=text2.len() {
                if text1[i - 1] == text2[j - 1] {
                    mem[i][j] = 1 + mem[i - 1][j - 1];
                } else {
                    mem[i][j] = mem[i - 1][j].max(mem[i][j - 1]);
                }
            }
        }
        mem.last().unwrap().last().unwrap().to_owned()
    }
}
