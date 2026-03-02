struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len() as i32;
        if m == 0 {
            return vec![];
        }
        let n = matrix[0].len() as i32;
        if n == 0 {
            return vec![];
        }
        let (mut top, mut bottom) = (0i32, m - 1);
        let (mut left, mut right) = (0i32, n - 1);
        let mut res = vec![];
        while top <= bottom && left <= right {
            // top row
            for col in left..=right {
                res.push(matrix[top as usize][col as usize]);
            }
            top += 1;
            // right col
            for row in top..=bottom {
                res.push(matrix[row as usize][right as usize]);
            }
            right -= 1;
            // bottom row
            if top <= bottom {
                for col in (left..=right).rev() {
                    res.push(matrix[bottom as usize][col as usize]);
                }
                bottom -= 1;
            }
            // left col
            if left <= right {
                for row in (top..=bottom).rev() {
                    res.push(matrix[row as usize][left as usize]);
                }
                left += 1;
            }
        }
        res
    }
}

fn main() {
    let res = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    println!("{res:?}");
}
