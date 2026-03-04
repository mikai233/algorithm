use crate::common::solution::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut l, mut r) = (0, m * n);
        while l < r {
            let mid = l + (r - l) / 2;
            let i = mid / n;
            let j = mid % n;
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => {
                    l = mid + 1;
                }
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Greater => {
                    r = mid;
                }
            }
        }
        false
    }
}
