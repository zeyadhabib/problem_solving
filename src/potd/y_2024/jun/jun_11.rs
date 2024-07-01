use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array_1(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mp = arr2
            .iter()
            .enumerate()
            .map(|(idx, num)| (*num, i32::MIN + idx as i32))
            .collect::<HashMap<i32, i32>>();

        let rev_mp = arr2
            .iter()
            .enumerate()
            .map(|(idx, num)| (i32::MIN + idx as i32, *num))
            .collect::<HashMap<i32, i32>>();

        arr1.iter_mut().for_each(|num| {
            *num = match mp.get(num) {
                Some(val) => *val,
                None => *num,
            }
        });

        arr1.sort();

        arr1.iter_mut().for_each(|num| {
            *num = match rev_mp.get(num) {
                Some(val) => *val,
                None => *num,
            }
        });

        arr1
    }
}

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut freq = arr1.iter().fold(vec![0; 1001], |mut freq, num| {
            freq[*num as usize] += 1;
            freq
        });

        let mut idx = 0 as usize;

        arr2.iter().for_each(|num| {
            for _ in 0..freq[*num as usize] {
                arr1[idx] = *num;
                idx += 1;
            }

            freq[*num as usize] = 0;
        });

        freq.iter().enumerate().for_each(|(num, f)| {
            for _ in 0..*f {
                arr1[idx] = num as i32;
                idx += 1;
            }
        });

        arr1
    }
}
