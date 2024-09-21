use crate::Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|num| num.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a, b| (b.to_owned() + &a).cmp(&(a.to_owned() + &b)));
        let ans = nums
            .iter()
            .fold(String::new(), |mut res, num| {
                res += num;
                res
            })
            .trim_start_matches('0')
            .to_owned();

        match ans.is_empty() {
            true => "0".to_string(),
            false => ans,
        }
    }
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn largest_number() {
        let nums = vec![3, 30, 34, 5, 9];
        assert_eq!(Solution::largest_number(nums), "9534330");
        let nums = vec![0, 0];
        assert_eq!(Solution::largest_number(nums), "0");
    }
}
