use crate::common::solution::Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            start: usize,
            candidates: &[i32],
            remian: i32,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if remian == 0 {
                ans.push(path.clone());
                return;
            }
            for i in start..candidates.len() {
                let val = candidates[i];
                if val > remian {
                    break;
                }
                path.push(val);
                backtrack(i, candidates, remian - val, path, ans);
                path.pop();
            }
        }
        candidates.sort_unstable();
        let mut path = vec![];
        let mut ans = vec![];
        backtrack(0, &candidates, target, &mut path, &mut ans);
        ans
    }
}
