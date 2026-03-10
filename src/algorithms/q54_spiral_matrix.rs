use crate::common::solution::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = vec![];
        let (mut top, mut bottom) = (0, m as i32 - 1);
        let (mut left, mut right) = (0, n as i32 - 1);
        while top <= bottom && left <= right {
            // top
            for col in left..=right {
                ans.push(matrix[top as usize][col as usize]);
            }
            top += 1;
            // right
            for row in top..=bottom {
                ans.push(matrix[row as usize][right as usize]);
            }
            right -= 1;
            // bottom
            if top <= bottom {
                for col in (left..=right).rev() {
                    ans.push(matrix[bottom as usize][col as usize]);
                }
                bottom -= 1;
            }
            // left
            if left <= right {
                for row in (top..=bottom).rev() {
                    ans.push(matrix[row as usize][left as usize]);
                }
                left += 1;
            }
        }
        ans
    }
}
