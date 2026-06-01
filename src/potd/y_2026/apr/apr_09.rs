use crate::Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();

        let mut prefix = vec![vec![1; m]; n];
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if i == 0 && j == 0 {
                    return;
                }

                if j == 0 {
                    prefix[i][j] = ((prefix[i - 1][m - 1] % 12345) * (grid[i - 1][m - 1] % 12345)) % 12345;
                } else {
                    prefix[i][j] = ((prefix[i][j - 1] % 12345) * (grid[i][j - 1] % 12345)) % 12345;
                }
            });
        });

        let mut suffix = vec![vec![1; m]; n];
        (0..n).rev().for_each(|i| {
            (0..m).rev().for_each(|j| {
                if i == n - 1 && j == m - 1 {
                    return;
                }

                if j == m - 1 {
                    suffix[i][j] = ((suffix[i + 1][0] % 12345) * (grid[i + 1][0] % 12345)) % 12345;
                } else {
                    suffix[i][j] = ((suffix[i][j + 1] % 12345) * (grid[i][j + 1] % 12345)) % 12345;
                }
            });
        });

        prefix
            .iter()
            .zip(suffix.iter())
            .map(|(p_r, s_r)| {
                p_r.iter()
                    .zip(s_r.iter())
                    .map(|(&p, &s)| ((p % 12345) * (s % 12345)) % 12345)
                    .collect()
            })
            .collect()
    }
}
