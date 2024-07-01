use crate::Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let check = |day: i32| -> bool {
            let (_, m_i) = bloom_day.iter().fold((0, 0), |(mut run, mut m_i), &bloom| {
                if bloom <= day {
                    run += 1;
                } else {
                    run = 0
                }

                if run == k {
                    run = 0;
                    m_i += 1;
                }

                (run, m_i)
            });

            m_i == m
        };

        let mut st = 0;
        let mut fin = 1e9 as i32;
        let mut result = -1;
        while st <= fin {
            let mid = (fin - st) / 2 + st;

            if check(mid) {
                result = mid;
                fin = mid - 1;
            } else {
                st = mid + 1;
            }
        }

        result
    }
}
