use crate::Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();
        
        let check = |force: i32| -> bool {
            position.iter().fold((0, i32::MIN / 2), |(balls, prev), &basket| {
                if basket - prev >= force {
                    (balls + 1, basket)
                } else {
                    (balls, prev)
                }
            }).0 >= m
        };


        let mut st = 0;
        let mut fin = position[position.len() - 1];

        let mut ans = i32::MAX;
        while st <= fin {
            let mid = (fin - st) / 2 + st;
            if check(mid) {
                ans = mid;
                st = mid + 1;
            } else {
                fin = mid - 1;
            }
        }

        ans
    }
}