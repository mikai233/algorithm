struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum.abs_diff(target) < closest.abs_diff(target) {
                    closest = sum;
                }
                if sum > target {
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    return target;
                }
            }
        }
        closest
    }
}
fn main() {
    println!("{}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!("{}", Solution::three_sum_closest(vec![0, 0, 0], 1));
    println!("{}", Solution::three_sum_closest(vec![10, 20, 30, 40, 50, 60, 70, 80, 90], 1));
}