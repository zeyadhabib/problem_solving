use crate::Solution;

use std::{cmp::max, collections::VecDeque};

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut deq: VecDeque<(i32, i32)> = VecDeque::new();
        for (i, &elem) in (0..k - 1).zip(nums.iter()) {
            while let Some(&(_idx, val)) = deq.back() {
                if elem >= val {
                    deq.pop_back();
                } else {
                    break;
                }
            }

            deq.push_back((i, elem));
        }

        let range = (k - 1)..(nums.len() as i32);
        let iter = nums.iter().skip((k - 1) as usize);
        for (i, &elem) in range.zip(iter) {
            if let Some(&(idx, _val)) = deq.front() {
                if idx == i - k {
                    deq.pop_front();
                }
            }

            while let Some(&(_idx, val)) = deq.back() {
                if elem >= val {
                    deq.pop_back();
                } else {
                    break;
                }
            }

            deq.push_back((i, elem));
            if let Some(&(_, val)) = deq.front() {
                ans.push(val);
            }
        }

        ans
    }
}

impl Solution {
    pub fn solve_3(
        energy_drink_a: &Vec<i32>,
        energy_drink_b: &Vec<i32>,
        memory: &mut Vec<Vec<i64>>,
        idx: usize,
        drink_type: bool,
    ) -> i64 {
        if idx >= energy_drink_a.len() {
            return 0;
        }

        if memory[idx][drink_type as usize] != -1 {
            return memory[idx][drink_type as usize];
        }

        memory[idx][drink_type as usize] = max(
            Self::solve_3(energy_drink_a, energy_drink_b, memory, idx + 1, drink_type),
            Self::solve_3(energy_drink_a, energy_drink_b, memory, idx + 2, !drink_type),
        ) + if drink_type {
            energy_drink_a[idx] as i64
        } else {
            energy_drink_b[idx] as i64
        };

        memory[idx][drink_type as usize]
    }

    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let mut memory = vec![vec![-1 as i64; 2]; energy_drink_a.len()];
        max(
            Self::solve_3(&energy_drink_a, &energy_drink_b, &mut memory, 0, false),
            Self::solve_3(&energy_drink_a, &energy_drink_b, &mut memory, 0, true),
        )
    }
}

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        let mut lookup = vec![0; n + 1];
        (1..n).for_each(|idx| {
            lookup[idx] = lookup[idx - 1] ^ arr[idx - 1];
        });

        queries.iter().map(|query| {
            lookup[query[0] as usize] ^ lookup[query[1] as usize + 1]
        }).collect::<Vec<_>>()
    }
}
