use crate::Solution;

use std::{cell::RefCell, rc::Rc};

// 1043. Partition Array for Maximum Sum
// TAGS: Dynamic Programming, DP, Top Down
impl Solution {
    pub fn solve(mem: Rc<RefCell<Vec<i32>>>, arr: Rc<Vec<i32>>, idx: usize, n: usize, k: usize) -> i32 {
        if idx == n {
            return 0;
        }

        let mem_arr = mem.borrow();
        if mem_arr[idx] != -1 {
            return mem_arr[idx];
        }
        
        drop(mem_arr);

        let mut ans = 0;
        let mut mx = arr[idx];

        for i in idx..(idx + k).min(n) {
            mx = mx.max(arr[i]);
            ans = ans.max((mx * (i - idx + 1) as i32) + Solution::solve(mem.clone(), arr.clone(), i + 1, n, k));
        }
        
        let mut mem_arr = mem.borrow_mut();
        mem_arr[idx] = ans;
        ans
    }

    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let arr = Rc::new(arr);
        let ref_mem: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![-1; n]));
        Solution::solve(ref_mem, arr, 0, n, k as usize)
    }
}
