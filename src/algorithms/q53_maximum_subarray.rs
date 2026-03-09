use crate::common::solution::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut best = nums[0];
        for &val in nums.iter().skip(1) {
            current = val.max(current + val);
            best = best.max(current);
        }
        best
    }
}
