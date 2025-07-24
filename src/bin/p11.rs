struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;
        while left < right {
            let h = height[left].min(height[right]);
            max_area = max_area.max(h * (right - left) as i32);
            while left < right && height[left] <= h {
                left += 1;
            }
            while left < right && height[right] <= h {
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