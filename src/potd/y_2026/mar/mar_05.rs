use crate::Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let set = s
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(idx, &val)| (idx % 2) as u8 + val)
            .filter(|&x| x != '1' as u8)
            .count() as i32;
        let clear = s
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(idx, &val)| ((idx + 1) % 2) as u8 + val)
            .filter(|&x| x != '1' as u8)
            .count() as i32;
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
