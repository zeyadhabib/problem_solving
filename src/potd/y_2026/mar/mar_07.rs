use crate::Solution;

use std::mem::swap;
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut set = s
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(idx, &val)| (idx % 2) as u8 + val)
            .filter(|&x| x != '1' as u8)
            .count() as i32;
        let mut clear = s
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(idx, &val)| ((idx + 1) % 2) as u8 + val)
            .filter(|&x| x != '1' as u8)
            .count() as i32;

        let mut ans = set.min(clear);

        for char in s.chars() {
            set -= (char == '0') as i32;
            clear -= (char == '1') as i32;

            if s.len() % 2 == 1 {
                set += (char == '1') as i32;
                clear += (char == '0') as i32;
            } else {
                set += (char == '0') as i32;
                clear += (char == '1') as i32;
            }

            ans = ans.min(set.min(clear));
            swap(&mut set, &mut clear);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_min_flips() {
        assert_eq!(Solution::min_flips("0100".to_string()), 1);
        assert_eq!(Solution::min_flips("1110".to_string()), 1);
        assert_eq!(Solution::min_flips("111000".to_string()), 2);
        assert_eq!(Solution::min_flips("01001001101".to_string()), 2);
    }
}
