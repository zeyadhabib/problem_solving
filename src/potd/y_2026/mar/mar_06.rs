use crate::Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let first_one = s.find('1');
        let last_one = s.rfind('1');
        if first_one.is_none() || last_one.is_none() {
            return false;
        }

        let first_one = first_one.unwrap();
        let last_one = last_one.unwrap();
        s[first_one..last_one]
            .as_bytes()
            .iter()
            .all(|a| *a == '1' as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_check_ones_segment() {
        assert_eq!(Solution::check_ones_segment("".to_string()), false);
        assert_eq!(Solution::check_ones_segment("1".to_string()), true);
        assert_eq!(Solution::check_ones_segment("110".to_string()), true);
        assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
        assert_eq!(Solution::check_ones_segment("1010101".to_string()), false);
    }
}
