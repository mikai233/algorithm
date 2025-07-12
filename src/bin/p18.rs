use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        if nums.len() < 3 {
            return result;
        }
        for i in 0..nums.len() - 3 {
            if nums[i] > target && nums[i] >= 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (i + 1)..nums.len() - 2 {
                if nums[i] + nums[j] > target && nums[i] + nums[j] >= 0 {
                    break;
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[i] + nums[j] + nums[left] + nums[right];
                    match sum.cmp(&target) {
                        Ordering::Less => {
                            left += 1;
                        }
                        Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right - 1] {
                                right -= 1;
                            }
                            left += 1;
                            right -= 1;
                        }
                        Ordering::Greater => {
                            right -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::four_sum(vec![0], 0));
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    println!("{:?}", Solution::four_sum(vec![2, 2, 2, 2, 2], 8));
}