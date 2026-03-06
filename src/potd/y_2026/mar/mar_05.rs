use crate::Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut clear = 0;
        let mut set = 0;

        let mut clear_expected = '0';
        let mut set_expected = '1';

        for (idx, char) in s.char_indices() {
            if char != clear_expected {
                clear += 1;
            }

            if char != set_expected {
                set += 1;
            }

            clear_expected = (((idx + 1) % 2) as u8 + '0' as u8) as char;

            set_expected = ((idx % 2) as u8 + '0' as u8) as char;
        }

        set.min(clear)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_min_operations() {
        assert_eq!(Solution::min_operations("10".to_string()), 0);
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }
}
