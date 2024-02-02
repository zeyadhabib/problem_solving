use crate::Solution;

// 1291. Sequential Digits
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let len_high = (f64::floor(f64::log10(high as f64)) as i32) + 1;
        let len_low = (f64::floor(f64::log10(low as f64)) as i32) + 1;

        for len in len_low..=len_high {
            for i in 1..=(9 - len + 1) {
                let mut ans = 0;
                for j in 0..len {
                    ans *= 10;
                    ans += i + j;
                }
                if ans >= low && ans <= high {
                    res.push(ans);
                }
            }
        }

        res
    }
}

// 605. Can Place Flowers
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let (mut ans, mut zero_runs) = flowerbed.iter().fold((0, 1), |(ans, zero_runs), num| {
            if *num == 0 {
                return (ans, zero_runs + 1);
            }
            (ans + (zero_runs - 1) / 2, 0)
        });

        zero_runs += 1;
        ans += (zero_runs - 1) / 2;
        ans >= n
    }
}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;

        for row in 0..n {
            let mut equal = vec![true; n];
            for i in 0..n {
                for col in 0..n {
                    equal[col] = equal[col] && (grid[row][i] == grid[i][col]);
                }
            }
            ans += equal.iter().fold(0, |sum, r#match| sum + (*r#match as i32));
        }

        ans
    }
}
