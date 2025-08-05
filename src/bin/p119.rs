struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut last_row = vec![1];
        for i in 1..=row_index {
            let mut current_row = Vec::with_capacity(i + 1);
            current_row.push(1);
            for j in 1..i {
                current_row.push(last_row[j - 1] + last_row[j]);
            }
            current_row.push(1);
            last_row = current_row;
        }
        last_row
    }
}

fn main() {
    println!("{:?}", Solution::get_row(0));
    println!("{:?}", Solution::get_row(3));
}