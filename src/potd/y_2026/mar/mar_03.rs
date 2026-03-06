use crate::Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut ans = vec!['0'];
        for _i in 1..n {
            let mut dup = ans.clone();
            for i in 0..dup.len() {
                dup[i] = match dup[i] {
                    '0' => '1',
                    _ => '0',
                }
            }

            dup.reverse();
            ans.push('1');
            ans.append(&mut dup);

            if ans.len() > k as usize {
                return ans[k as usize];
            }
        }

        ans[(k - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_find_kth_bit() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
