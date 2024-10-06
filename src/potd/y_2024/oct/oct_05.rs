use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let (freq_1, mut freq_2) = (0..s1.len()).fold(
            (vec![0; 26], vec![0; 26]),
            |(mut freq_1, mut freq_2), idx| {
                freq_1[s1.as_bytes()[idx] as usize - 'a' as usize] += 1;
                freq_2[s2.as_bytes()[idx] as usize - 'a' as usize] += 1;
                (freq_1, freq_2)
            },
        );

        let mut num_eq = freq_1
            .iter()
            .zip(freq_2.iter())
            .fold(0, |sum, (&a, &b)| sum + (b >= a) as usize);

        if num_eq == 26 {
            return true;
        }

        for idx in s1.len()..s2.len() {
            let char_idx_add = s2.as_bytes()[idx] as usize - 'a' as usize;
            let char_idx_remove = s2.as_bytes()[idx - s1.len()] as usize - 'a' as usize;

            if freq_1[char_idx_remove] == freq_2[char_idx_remove] {
                num_eq -= 1;
            }

            freq_2[char_idx_add] += 1;
            freq_2[char_idx_remove] -= 1;

            if freq_1[char_idx_add] == freq_2[char_idx_add] {
                num_eq += 1;
            }

            println!("{num_eq}");

            if num_eq == 26 {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn check_inclusion() {
        let s1 = "abc".to_owned();
        let s2 = "ccccbbbbaaaa".to_owned();

        let res = Solution::check_inclusion(s1, s2);
        assert!(!res);
    }
}
