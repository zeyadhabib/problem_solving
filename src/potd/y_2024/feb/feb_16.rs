use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut freq = arr
            .iter()
            .fold(&mut HashMap::new(), |map, num| {
                map.entry(num).and_modify(|e| *e += 1).or_insert(1);
                map
            })
            .iter()
            .map(|(_, f)| *f)
            .collect::<Vec<i32>>();
        freq.sort();

        let mut k = k;
        for (i, num) in freq.iter().enumerate() {
            if k - *num as i32 >= 0 {
                k -= *num as i32;
            } else {
                return (freq.len() - i) as i32;
            }
        }

        0
    }
}
