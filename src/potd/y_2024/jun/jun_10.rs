use crate::Solution;

impl Solution {
    pub fn height_checker_1(heights: Vec<i32>) -> i32 {
        heights
            .iter()
            .fold(
                vec![0; (heights.iter().max().unwrap().to_owned() as usize) + 1],
                |mut freq, num| {
                    freq[*num as usize] += 1;
                    freq
                },
            )
            .iter()
            .enumerate()
            .fold((0, 0 as usize), |(mut ans, mut idx), (num, freq)| {
                for _ in 0..*freq {
                    ans += (heights[idx] != num as i32) as i32;
                    idx += 1;
                }

                (ans, idx)
            })
            .0
    }
}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();

        expected
            .iter()
            .zip(heights.iter())
            .map(|(num1, num2)| (num1 != num2) as i32)
            .sum()
    }
}
