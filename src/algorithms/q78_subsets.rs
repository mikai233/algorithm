use crate::common::solution::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(start: usize, nums: &[i32], ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            ans.push(path.clone());
            for i in start..nums.len() {
                path.push(nums[i]);
                dfs(i + 1, nums, ans, path);
                path.pop();
            }
        }
        let mut ans = vec![];
        let mut path = vec![];
        dfs(0, &nums, &mut ans, &mut path);
        ans
    }
}
