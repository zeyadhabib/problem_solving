use crate::Solution;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Solution {

    pub fn slv_2 (i: i32, n: i32, mem: Rc<RefCell<HashMap<i32, i32>>>) -> i32 {
        if i == n {
            return 0;
        }

        if i > n {
            return i32::MAX / 4;
        }

        if let Some(sol) = mem.borrow().get(&i) {
            return *sol;
        }

        let mut ans = i32::MAX / 4;
        let fin = f64::sqrt(n as f64).ceil() as i32;
        for j in (1..=fin).rev() {
            ans = ans.min(1 + Solution::slv_2(i + j * j, n, mem.clone()));
        }

        mem.borrow_mut().insert(i, ans);
        ans
    }

    pub fn num_squares(n: i32) -> i32 {
        let mem = Rc::new(RefCell::new(HashMap::new()));
        Solution::slv_2(0, n, mem)
    }
}