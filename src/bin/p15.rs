use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    Ordering::Less => left += 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                    }
                    Ordering::Greater => right -= 1,
                }
            }
        }
        result
    }
}
fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));// -4 -1 -1 0 1 2
    println!("{:?}", Solution::three_sum(vec![0, 1, 1]));
    println!("{:?}", Solution::three_sum(vec![0, 0, 0]));
    println!("{:?}", Solution::three_sum(vec![1, -1, -1, 0]));
    println!("{:?}", Solution::three_sum(vec![-2, 0, 1, 1, 2]));
}