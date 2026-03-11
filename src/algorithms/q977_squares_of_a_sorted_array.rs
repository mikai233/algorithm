use crate::common::solution::Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut l = 0 as i32;
        let mut r = nums.len() as i32 - 1;
        let mut i = r as usize;
        while l <= r {
            let ni = nums[l as usize] * nums[l as usize];
            let nj = nums[r as usize] * nums[r as usize];
            if ni <= nj {
                ans[i] = nj;
                r -= 1;
            } else {
                ans[i] = ni;
                l += 1;
            }
            i -= 1;
        }
        ans
    }
}
