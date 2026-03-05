use crate::common::solution::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let target = target as i64;
        let mut left = 0;
        let mut best: Option<usize> = None;
        let mut sum = 0i64;

        for right in 0..nums.len() {
            sum += nums[right] as i64;
            while sum >= target {
                best = Some(best.unwrap_or(usize::MAX).min(right - left + 1));
                sum -= nums[left] as i64;
                left += 1;
            }
        }
        best.unwrap_or_default() as i32
    }
}
