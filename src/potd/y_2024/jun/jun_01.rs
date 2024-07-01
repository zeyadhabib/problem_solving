use crate::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let last = s.chars().next().unwrap() as i32;
        s.chars().fold((last, 0), |(mut last, mut sum), char_i| {
            sum += (char_i as i32 - last).abs();
            last = char_i as i32;
            (last, sum)
        }).1
    }
}
