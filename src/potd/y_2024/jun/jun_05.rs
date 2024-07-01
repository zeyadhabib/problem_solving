use crate::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        words
            .iter()
            .fold(vec![(true, i32::MAX); 26], |freq, r#str| {
                let mut local_freq = vec![(false, 0); 26];
                str.chars().for_each(|ch| {
                    let idx = ch as usize - 'a' as usize;
                    local_freq[idx].0 = true;
                    local_freq[idx].1 += 1;
                });

                freq.iter()
                    .zip(local_freq.iter())
                    .map(|(a, b)| -> (bool, i32) { (a.0 & b.0, a.1.min(b.1)) })
                    .collect()
            })
            .iter()
            .enumerate()
            .for_each(|(idx, (included, freq))| {
                if *included {
                    for _ in 0..(*freq) {
                        let mut r#str = String::new();
                        r#str.push((idx as u8 + 'a' as u8) as char);
                        result.push(r#str);
                    }
                }
            });

        result
    }
}
