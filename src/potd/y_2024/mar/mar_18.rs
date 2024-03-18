use crate::Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort();
        
        let mut ans = 1;
        let mut intersection = points[0].clone();

        for point in points {
            let start = point[0].max(intersection[0]);
            let end = point[1].min(intersection[1]);

            if start > end {
                ans += 1;
                intersection = point;
            } else {
                intersection = vec![start, end];
            }
        }
        
        ans
    }
}