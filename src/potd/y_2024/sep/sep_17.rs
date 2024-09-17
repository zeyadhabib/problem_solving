use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let words_1 = s1.split(' ');
        let words_2 = s2.split(' ');
        let mut map_1 = HashMap::new();
        let mut map_2 = HashMap::new();

        words_1.for_each(|word| {
            *map_1.entry(word).or_insert(0) += 1;
        });
        words_2.for_each(|word| {
            *map_2.entry(word).or_insert(0) += 1;
        });

        let mut ans = Vec::new();
        for (&word, &freq) in map_1.iter() {
            if freq == 1 && !map_2.contains_key(word) {
                ans.push(word.to_string());
            }
        }

        for (word, freq) in map_2 {
            if freq == 1 && !map_1.contains_key(word) {
                ans.push(word.to_string());
            }
        }

        ans
    }
}
