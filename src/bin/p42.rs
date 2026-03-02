struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut res = 0;
        while l < r {
            left_max = left_max.max(height[l]);
            right_max = right_max.max(height[r]);
            if left_max <= right_max {
                res += left_max - height[l];
                l += 1;
            } else {
                res += right_max - height[r];
                r -= 1;
            }
        }
        res
    }
}

fn main() {
    let res = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
    println!("{res}");
}
