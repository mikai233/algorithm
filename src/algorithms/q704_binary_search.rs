use crate::common::solution::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Less => {
                    l = mid + 1;
                }
                std::cmp::Ordering::Equal => {
                    return mid as i32;
                }
                std::cmp::Ordering::Greater => {
                    r = mid;
                }
            }
        }
        -1
    }
}
