use crate::common::solution::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = l + (r - l) / 2;
            match target.cmp(&nums[mid]) {
                std::cmp::Ordering::Less => {
                    r = mid;
                }
                std::cmp::Ordering::Equal => {
                    return mid as i32;
                }
                std::cmp::Ordering::Greater => {
                    l = mid + 1;
                }
            }
        }
        l as i32
    }
}
