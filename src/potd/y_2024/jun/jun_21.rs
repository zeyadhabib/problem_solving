use crate::Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let total = customers
            .iter()
            .zip(grumpy.iter())
            .map(|(&customers, &grumpy)| customers * ((grumpy + 1) % 2))
            .sum::<i32>();

        (0..customers.len())
            .fold((total, total), |(mut sum, ans), idx| {
                sum += customers.get(idx).unwrap_or(&0) * grumpy.get(idx).unwrap_or(&0);
                sum -= customers.get(idx - minutes as usize).unwrap_or(&0)
                    * grumpy.get(idx - minutes as usize).unwrap_or(&0);
                (sum, ans.max(sum))
            })
            .1
    }
}
