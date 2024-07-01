use crate::Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut mp = BTreeMap::new();
        let group_size = group_size as usize;

        if hand.len() % group_size != 0 {
            return false;
        }

        for num in hand {
            mp.entry(num)
                .and_modify(|value| {
                    *value += 1;
                })
                .or_insert(1);
        }

        let keys = mp.keys().cloned().collect::<Vec<i32>>();

        for num in keys {
            let freq = mp.get(&num).unwrap().to_owned();
            if freq == 0 {
                continue;
            }

            for incr in 0..(group_size as i32) {
                if let Some(next_freq) = mp.get(&(num + incr)) {
                    if next_freq >= &freq {
                        mp.entry(num + incr).and_modify(|e| *e -= freq);
                    }
                }

                return false;
            }
        }

        return true;
    }
}
