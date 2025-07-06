struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || s.len() <= num_rows {
            return s;
        }
        let cycle_len = num_rows * 2 - 2;
        let mut results = vec!["".to_string(); num_rows];
        for (index, ch) in s.chars().enumerate() {
            let pos = index % cycle_len;
            if pos < num_rows {
                results[pos].push(ch);
            } else {
                results[cycle_len - pos].push(ch);
            }
        }
        results.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 4));
}