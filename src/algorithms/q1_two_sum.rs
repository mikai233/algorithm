use std::collections::HashMap;

use crate::common::solution::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visted = HashMap::new();
        let mut ans = vec![];
        for (i, n) in nums.iter().enumerate() {
            match visted.get(&(target - n)) {
                Some(&pre) => {
                    ans.extend([i as i32, pre as i32]);
                    break;
                }
                None => {
                    visted.insert(n, i);
                }
            }
        }
        ans
    }
}
