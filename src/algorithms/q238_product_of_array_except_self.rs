use crate::common::solution::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; nums.len()];
        let mut prefix = 1;
        for (i, n) in nums.iter().enumerate() {
            ans[i] = prefix;
            prefix *= n;
        }
        let mut suffix = 1;
        for (i, n) in nums.iter().enumerate().rev() {
            ans[i] *= suffix;
            suffix *= n;
        }
        ans
    }
}
