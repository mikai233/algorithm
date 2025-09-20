struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut current = String::with_capacity((n * 2) as usize);
        Self::backtrack(&mut result, &mut current, n, 0, 0);
        result
    }

    fn backtrack(
        result: &mut Vec<String>,
        current: &mut String,
        n: i32,
        open_count: i32,
        close_count: i32,
    ) {
        if current.len() == (n as usize) * 2 {
            result.push(current.to_string());
            return;
        }
        if open_count < n {
            current.push('(');
            Self::backtrack(result, current, n, open_count + 1, close_count);
            current.pop();
        }
        if close_count < open_count {
            current.push(')');
            Self::backtrack(result, current, n, open_count, close_count + 1);
            current.pop();
        }
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
