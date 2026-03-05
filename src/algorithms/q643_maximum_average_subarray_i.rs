use crate::common::solution::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut window_sum: i64 = nums[..k].iter().map(|&v| v as i64).sum();
        let mut best_sum = window_sum;
        for i in k..nums.len() {
            window_sum += nums[i] as i64;
            window_sum -= nums[i - k] as i64;
            best_sum = best_sum.max(window_sum);
        }
        best_sum as f64 / k as f64
    }
}
