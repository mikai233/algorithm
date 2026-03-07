use crate::common::solution::Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;
            if i < 0 || i >= m || j < 0 || j >= n {
                return;
            }
            let x = i as usize;
            let y = j as usize;
            if grid[x][y] != '1' {
                return;
            }
            grid[x][y] = '0';
            dfs(grid, i + 1, j);
            dfs(grid, i - 1, j);
            dfs(grid, i, j + 1);
            dfs(grid, i, j - 1);
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    ans += 1;
                    dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        ans
    }
}
