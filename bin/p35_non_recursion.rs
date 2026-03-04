struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid];
            if mid_val < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 5));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 2));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 7));
}
