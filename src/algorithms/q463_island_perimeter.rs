use crate::common::solution::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut perimeter = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    perimeter += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        perimeter -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }
        perimeter
    }
}
