use crate::common::solution::Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = vec![];
        let mut path = vec![];
        fn dfs(start: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            ans.push(path.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }
                path.push(nums[i]);
                dfs(i + 1, nums, path, ans);
                path.pop();
            }
        }
        dfs(0, &nums, &mut path, &mut ans);
        ans
    }
}
