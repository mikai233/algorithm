use crate::common::solution::Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut best = nums[0];
        for &num in &nums[1..] {
            if best.abs() > num.abs() || (best.abs() == num.abs() && num > best) {
                best = num;
            }
        }
        best
    }
}
