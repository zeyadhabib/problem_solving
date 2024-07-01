use crate::Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut idx_s = 0 as usize;
        let mut idx_t = 0 as usize;

        let vec_s = s.as_bytes();
        let vec_t = t.as_bytes();

        while idx_s < s.len() && idx_t < t.len() {
            idx_t += (vec_s[idx_s] == vec_t[idx_t]) as usize;
            idx_s += 1;
        }

        (t.len() - idx_t) as i32
    }
}

// 3152. Special Array II
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix = vec![0; nums.len()]; 
        (1..nums.len()).for_each(|idx| {
            prefix[idx] = match (nums[idx] ^ nums[idx - 1]) & 1 {
                0 => 0,
                1 => prefix[idx - 1] + 1,
                _ => 0
            }
        });        

        queries.iter().map(|query| -> bool {
            let st = query[0] as usize;
            let fin = query[1] as usize;
            (prefix[fin] - prefix[st]) == (fin - st)
        }).collect()
    }
}