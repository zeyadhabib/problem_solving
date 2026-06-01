use crate::Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let num_digits = n.ilog2() + 1;
        let full_complement = !n;
        let mask = (1 << num_digits) - 1;
        full_complement & mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_bitwise_complement() {
        assert_eq!(Solution::bitwise_complement(0), 1);
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(7), 0);
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}