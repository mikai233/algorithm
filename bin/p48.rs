struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // matrix.length == matrix[i].length
        let n = matrix.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for ele in matrix {
            ele.reverse();
        }
    }
}

fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut matrix = matrix.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
    Solution::rotate(&mut matrix);
    for ele in matrix {
        println!("{ele:?}")
    }
}
