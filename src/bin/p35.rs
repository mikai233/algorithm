use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search(&nums, target, 0, nums.len()) as i32
    }

    fn binary_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> usize {
        if left == right {
            return left;
        }
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Less => {
                Self::binary_search(nums, target, mid + 1, right)
            }
            Ordering::Equal => {
                mid
            }
            Ordering::Greater => {
                Self::binary_search(nums, target, left, mid)
            }
        }
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 5));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 2));
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 7));
}