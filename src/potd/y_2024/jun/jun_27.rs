use crate::Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut degrees = vec![0; edges.len() + 2];

        edges.iter().for_each(|edge| {
            degrees[edge[0] as usize] += 1;
            degrees[edge[1] as usize] += 1;

            if degrees[edge[0] as usize] == edges.len() {
                ans = edge[0];
            }

            if degrees[edge[1] as usize] == edges.len() {
                ans = edge[1];
            }
        });

        ans
    }
}
