use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;
        while left < right {
            let l = height[left];
            let r = height[right];
            let height = std::cmp::min(l, r);
            max_area = max(max_area, height * (right - left) as i32);
            if l < r {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
    }
}
fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", Solution::max_area(vec![1, 1]));
}