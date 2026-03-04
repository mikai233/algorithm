use crate::common::solution::Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        fn dfs(
            i: usize,
            j: usize,
            matrix: &Vec<Vec<i32>>,
            dp: &mut Vec<Vec<i32>>,
            dirs: &[(i32, i32)],
        ) -> i32 {
            let record = dp[i][j];
            if record != 0 {
                return record;
            }
            let m = matrix.len() as i32;
            let n = matrix[0].len() as i32;
            let mut best = 1;
            let val = matrix[i][j];
            for &(dx, dy) in dirs {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;
                if ni >= 0 && ni < m && nj >= 0 && nj < n {
                    let ui = ni as usize;
                    let uj = nj as usize;
                    if matrix[ui][uj] > val {
                        best = best.max(1 + dfs(ui, uj, matrix, dp, dirs))
                    }
                }
            }
            dp[i][j] = best;
            best
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans = ans.max(dfs(i, j, &matrix, &mut dp, &dirs));
            }
        }
        ans
    }
}
