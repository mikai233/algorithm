use crate::common::solution::Solution;

impl Solution {
    pub fn search_33(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            let mid_val = nums[mid];
            if mid_val == target {
                return mid as i32;
            }
            let left_val = nums[l as usize];
            let right_val = nums[r as usize];
            if left_val <= mid_val {
                if left_val <= target && target < mid_val {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                if mid_val < target && target <= right_val {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}
