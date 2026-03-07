use crate::common::solution::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ans = vec![];
        fn backtrack(
            start: usize,
            n: usize,
            k: usize,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == k {
                ans.push(path.clone());
                return;
            }
            let need = k - path.len();
            for i in start..=(n - need + 1) {
                path.push(i as i32);
                backtrack(i + 1, n, k, path, ans);
                path.pop();
            }
        }
        backtrack(1, n as usize, k as usize, &mut path, &mut ans);
        ans
    }
}
