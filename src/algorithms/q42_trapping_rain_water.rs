use crate::common::solution::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut l_max, mut r_max) = (0, 0);
        let mut ans = 0;
        while l < r {
            l_max = l_max.max(height[l]);
            r_max = r_max.max(height[r]);
            if l_max <= r_max {
                ans += l_max - height[l];
                l += 1;
            } else {
                ans += r_max - height[r];
                r -= 1;
            }
        }
        ans
    }
}
