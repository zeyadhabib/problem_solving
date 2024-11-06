use std::{cell::RefCell, rc::Rc};

use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .fold(
                (1, Rc::new(RefCell::new(vec![1; nums.len()]))),
                |(mut ans, mem), i| {
                    let mut mem_mut = mem.borrow_mut();
                    for j in 0..i {
                        if nums[i] > nums[j] {
                            mem_mut[i] = mem_mut[i].max(mem_mut[j] + 1);
                            ans = ans.max(mem_mut[i]);
                        }
                    }
                    drop(mem_mut);

                    (ans, mem)
                },
            )
            .0
    }
}
