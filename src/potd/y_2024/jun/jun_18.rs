use crate::Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs = difficulty
            .iter()
            .zip(profit.iter())
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<_>>();
        jobs.sort();

        for i in 1..jobs.len() {
            jobs[i].1 = jobs[i].1.max(jobs[i - 1].1);
        }

        let mut total_profit = 0;
        for &ability in &worker {
            let mut ans = i32::MAX;
            let mut st = 0;
            let mut r#fn = jobs.len() as i32 - 1;

            while st <= r#fn {
                let mid = (r#fn - st) / 2 + st;
                if jobs[mid as usize].0 <= ability {
                    ans = mid;
                    st = mid + 1;
                } else {
                    r#fn = mid - 1;
                }
            }

            if ans != i32::MAX {
                total_profit += jobs[ans as usize].1;
            }
        }

        total_profit
    }
}
