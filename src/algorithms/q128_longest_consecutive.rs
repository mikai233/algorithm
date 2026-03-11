use crate::common::solution::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut best = 0;
        for &num in &nums {
            if !nums.contains(&(num - 1)) {
                let mut curr = num;
                let mut len = 0;
                while nums.contains(&curr) {
                    len += 1;
                    curr += 1;
                }
                best = best.max(len);
            }
        }
        best
    }
}
