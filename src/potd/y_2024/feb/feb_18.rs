use crate::Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        let mut counts = vec![0; n as usize];
        let mut rooms = BinaryHeap::new();
        for i in 0..n as usize {
            rooms.push(Reverse((0 as usize, i)));
        }

        meetings.sort();
        for meeting in meetings {
            while rooms.peek().unwrap().0 .0 < meeting[0] as usize {
                let (_available, idx) = rooms.pop().unwrap().0;
                rooms.push(Reverse((meeting[0] as usize, idx)));
            }

            let (available, idx) = rooms.pop().unwrap().0;
            rooms.push(Reverse((
                available - meeting[0] as usize + meeting[1] as usize,
                idx,
            )));
            counts[idx] += 1;
        }

        counts
            .iter()
            .enumerate()
            .fold((0, 0), |(maxi, maxv), (idx, cnt)| {
                if maxv < *cnt {
                    (idx, *cnt)
                } else {
                    (maxi, maxv)
                }
            })
            .0 as i32
    }
}
