use crate::common::solution::Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let target = n - k as usize;
        let (mut left, mut right) = (0, n - 1);
        while left <= right {
            let (lt, gt) = Solution::partition3(&mut nums, left, right);
            if target < lt {
                right = lt - 1;
            } else if target > gt {
                left = gt + 1;
            } else {
                break;
            }
        }
        nums[target]
    }

    fn partition3(nums: &mut Vec<i32>, left: usize, right: usize) -> (usize, usize) {
        let mid = left + (right - left) / 2;
        nums.swap(mid, right);
        let mut lt = left;
        let mut i = left;
        let mut gt = right;
        let pivot = nums[right];
        while i <= gt {
            match nums[i].cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    nums.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    nums.swap(gt, i);
                    if gt == 0 {
                        break;
                    }
                    gt -= 1;
                }
            }
        }
        (lt, gt)
    }
}
