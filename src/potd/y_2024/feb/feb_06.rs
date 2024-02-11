use crate::Solution;

use std::collections::HashMap;

// 49. Group Anagrams
// TAGS: strings, hash maps
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mp = HashMap::<Vec<i32>, Vec<String>>::new();
        for str in strs.iter() {
            let fq = str.chars().fold(vec![0; 26], |mut f_arr, c| {
                f_arr[c as usize - 'a' as usize] += 1;
                f_arr
            });

            mp.entry(fq)
                .and_modify(|strings| strings.push(str.to_owned()))
                .or_insert(vec![str.to_owned()]);
        }

        mp.values().fold(Vec::new(), |mut ans, str_array| {
            ans.push(str_array.to_owned());
            ans
        })
    }
}
