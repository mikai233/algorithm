struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut rows = Vec::with_capacity(num_rows);
        if num_rows >= 1 {
            rows.push(vec![1]);
        }
        for i in 1..num_rows {
            let mut row = Vec::with_capacity(i + 1);
            row.push(1);
            for j in 1..i {
                row.push(rows[i - 1][j - 1] + rows[i - 1][j]);
            }
            row.push(1);
            rows.push(row);
        }
        rows
    }
}

fn main() {
    println!("{:?}", Solution::generate(1));
    println!("{:?}", Solution::generate(5));
}