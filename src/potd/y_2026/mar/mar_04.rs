use crate::Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let row_sum = mat
            .iter()
            .map(|row| row.into_iter().sum::<i32>())
            .collect::<Vec<i32>>();
        let col_sum = (0..n).fold(vec![0; n], |mut sum, idx| {
            sum[idx] = mat.iter().map(|row| row[idx]).sum::<i32>();
            sum
        });

        (0..m).fold(0, |acc, i| {
            let sub = (0..n).fold(0, |acc_i, j| {
                if mat[i][j] == 1 && row_sum[i] == 1 && col_sum[j] == 1 {
                    acc_i + 1
                } else {
                    acc_i
                }
            });

            acc + sub
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_num_special() {
        let mat_1 = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        let mat_2 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        assert_eq!(Solution::num_special(mat_1), 1);
        assert_eq!(Solution::num_special(mat_2), 3);
    }
}
