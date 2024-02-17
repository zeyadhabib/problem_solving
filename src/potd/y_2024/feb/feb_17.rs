use crate::Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let res =
            heights
                .iter()
                .enumerate()
                .try_fold(BinaryHeap::new(), |mut diffs, (idx, height)| {
                    if let Some(nxt) = heights.get(idx + 1) {
                        let diff = *nxt - *height;
                        if diff > 0 {
                            diffs.push(Reverse(diff));
                        }

                        if diffs.len() as i32 > ladders {
                            bricks -= diffs.pop().unwrap().0;
                        }

                        if bricks < 0 {
                            return Err(idx);
                        }
                    }

                    Ok(diffs)
                });

        match res {
            Ok(_) => heights.len() as i32 - 1,
            Err(ans) => ans as i32,
        }
    }
}
