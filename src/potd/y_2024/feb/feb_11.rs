use crate::Solution;

use std::{cell::RefCell, rc::Rc};

type Vec1 = Vec<i32>;
type Vec2 = Vec<Vec1>;
type Vec3 = Vec<Vec2>;
pub struct Ctx(usize, usize, Rc<RefCell<Vec3>>, Rc<Vec2>);

// 1463. Cherry Pickup II
// TAGS: DP, DFS, Memoization, top-down, tuple struct
impl Solution {
    pub fn slv_3(r_i: usize, r1_j: i32, r2_j: i32, Ctx(m, n, mem, grid): Ctx) -> i32 {
        if r_i >= m || r1_j >= n as i32 || r2_j >= n as i32 || r1_j < 0 || r2_j < 0 {
            return 0;
        }

        let imut_mem = mem.borrow();
        if imut_mem[r_i][r1_j as usize][r2_j as usize] != -1 {
            return imut_mem[r_i][r1_j as usize][r2_j as usize];
        }
        drop(imut_mem);

        let mut ans = 0;
        let r1 = grid[r_i][r1_j as usize];
        let r2 = if r1_j != r2_j {
            grid[r_i][r2_j as usize]
        } else {
            0
        };

        for off1 in -1..=1 {
            for off2 in -1..=1 {
                ans = ans.max(
                    r1 + r2
                        + Solution::slv_3(
                            r_i + 1,
                            r1_j + off1,
                            r2_j + off2,
                            Ctx(m, n, mem.clone(), grid.clone()),
                        ),
                );
            }
        }

        let mut mut_mem = mem.borrow_mut();
        mut_mem[r_i][r1_j as usize][r2_j as usize] = ans;
        ans
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mem = vec![vec![vec![-1; n]; n]; m];
        Solution::slv_3(
            0,
            0,
            n as i32 - 1,
            Ctx(m, n, Rc::new(RefCell::new(mem)), Rc::new(grid)),
        )
    }
}