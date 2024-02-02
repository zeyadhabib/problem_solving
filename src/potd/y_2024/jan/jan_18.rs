use crate::Solution;

/*
70. Climbing Stairs
Tags: Dynamic Programming, Fibonacci
*/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        
        if n == 1 {
            return 1;
        }

        if n == 2 {
            return 2;
        }
        
        let mut f0 = 1;
        let mut f1 = 2;

        for _i in 3..=n {
            let f2 = f0 + f1;
            f0 = f1;
            f1 = f2;
        }

        return f1;
    }
}


/*
Leeetcode Problem 2171. Removing Minimum Number of Magic Beans
Tags: Sorting, Prefix Sum, Suffix Sum, Greedy
*/
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let n = beans.len();
        let mut prefix: i64 = 0;
        let mut suffix: i64 = beans.iter().fold(0, |acc, x| acc + (*x as i64));
        let mut beans = beans;
        beans.sort_unstable();
        let mut min = i64::MAX;

        for i in 0..beans.len() {
            suffix -= beans[i] as i64;
            println!("{prefix} {suffix} {min}");
            min = min.min(prefix + suffix - (n - i - 1) as i64 * (beans[i] as i64));
            prefix += beans[i] as i64;
        }

        min
    }
}