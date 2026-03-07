use crate::common::solution::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];
        fn backtrack(
            nums: &[i32],
            path: &mut Vec<i32>,
            used: &mut [bool],
            ans: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                ans.push(path.clone());
            }
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                backtrack(nums, path, used, ans);
                path.pop();
                used[i] = false;
            }
        }
        backtrack(&nums, &mut path, &mut used, &mut ans);
        ans
    }
}
