use crate::common::solution::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_areas = 0;
        while left < right {
            let h = height[left].min(height[right]);
            max_areas = max_areas.max((right - left) as i32 * h);
            while left < right && height[left] <= h {
                left += 1;
            }
            while left < right && height[right] <= h {
                right -= 1;
            }
        }
        max_areas
    }
}
