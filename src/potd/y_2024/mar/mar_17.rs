use crate::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut idx = 0;
        while idx < intervals.len() {
            let start = intervals[idx][0].max(new_interval[0]);
            let finish = intervals[idx][1].min(new_interval[1]);

            if new_interval[1] < intervals[idx][0] || start <= finish {
                break;
            } else {
                ans.push(intervals[idx].clone());
                idx += 1;
            }
        }

        ans.push(new_interval);

        while idx < intervals.len() {
            let last = ans.last().unwrap();
            let start = intervals[idx][0].max(last[0]);
            let finish = intervals[idx][1].min(last[1]);
            
            if start > finish {
                ans.push(intervals[idx].clone());
            } else {
                let last = ans.pop().unwrap();
                let start_new = last[0].min(intervals[idx][0]);
                let finish_new = last[1].max(intervals[idx][1]);
                ans.push(vec![start_new, finish_new]);
            }

            idx += 1;
        }
        
        ans
    }
}