use crate::common::solution::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut i = 0;
        let mut res = vec![];
        while i < nums.len() {
            let mut j = i;
            while j + 1 < nums.len() && nums[j] + 1 == nums[j + 1] {
                j += 1;
            }
            if i == j {
                res.push(format!("{}", nums[i]));
            } else {
                res.push(format!("{}->{}", nums[i], nums[j]));
            }
            i = j + 1;
        }
        res
    }
}
