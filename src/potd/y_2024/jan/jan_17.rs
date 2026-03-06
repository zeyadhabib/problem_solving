use crate::Solution;

use std::collections::{HashMap, HashSet};


/*
1207. Unique Number of Occurrences
TAGS: HashMap, HashSet, Iterators, fold, entry, or_insert, all, insert
*/

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut ff = HashSet::new();
        arr.iter().fold(HashMap::new(), |mut acc, &x| {
            acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
            acc
        }).values().all(|num| ff.insert(num))
    }
}