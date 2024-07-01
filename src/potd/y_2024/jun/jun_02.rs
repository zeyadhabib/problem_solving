use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let fin = n / 2;
        for i in 0..fin {
            let tmp = s[i];
            s[i] = s[n - i - 1];
            s[n - i - 1] = tmp;
        }
    }
}