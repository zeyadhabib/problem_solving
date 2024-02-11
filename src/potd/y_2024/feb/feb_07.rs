use crate::Solution;

use std::collections::HashMap;

// 451. Sort Characters By Frequency
// TAGS: Frequency array, strings
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut fq = s.chars().fold(HashMap::new(), |mut f_arr, ch| {
            f_arr.entry(ch).and_modify(|e| *e += 1).or_insert(1);
            f_arr
        });

        let mut vec = fq
            .drain()
            .collect::<Vec<(char, i32)>>();

        vec.sort_by(|(l_ch, l_fq), (r_ch, r_fq)| r_fq.cmp(l_fq).then(l_ch.cmp(r_ch)));

        vec.iter().fold(String::new(), |mut ans, (ch, fq)| {
            for _i in 0..*fq {
                ans.push(ch.to_owned());
            }
            ans
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        // Test case 1
        let s1 = String::from("tree");
        let expected1 = String::from("eert");
        assert_eq!(Solution::frequency_sort(s1), expected1);

        // Test case 2
        let s2 = String::from("cccaaa");
        let expected2 = String::from("aaaccc");
        assert_eq!(Solution::frequency_sort(s2), expected2);

        // Test case 3
        let s3 = String::from("Aabb");
        let expected3 = String::from("bbAa");
        assert_eq!(Solution::frequency_sort(s3), expected3);
    }
}
