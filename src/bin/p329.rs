struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        if n == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; n]; m];
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn dfs(
            i: usize,
            j: usize,
            matrix: &Vec<Vec<i32>>,
            dp: &mut Vec<Vec<i32>>,
            dirs: &[(i32, i32)],
        ) -> i32 {
            if dp[i][j] != 0 {
                return dp[i][j];
            }

            let m = matrix.len() as i32;
            let n = matrix[0].len() as i32;
            let val = matrix[i][j];
            let mut best = 1;
            for (dx, dy) in dirs {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;
                if ni >= 0 && ni < m && nj >= 0 && nj < n {
                    let ui = ni as usize;
                    let uj = nj as usize;
                    if matrix[ui][uj] > val {
                        best = best.max(1 + dfs(ui, uj, matrix, dp, dirs));
                    }
                }
            }
            dp[i][j] = best;
            best
        }

        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                ans = ans.max(dfs(i, j, &matrix, &mut dp, &dirs))
            }
        }

        ans
    }
}

fn main() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    let n = Solution::longest_increasing_path(matrix);
    println!("{n}");
}
