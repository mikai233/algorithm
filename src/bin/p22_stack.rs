struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut stack = vec![("".to_string(), 0, 0)];
        while let Some((current, open, close)) = stack.pop() {
            if current.len() == (n as usize) * 2 {
                result.push(current.clone());
                continue;
            }
            if open < n {
                let mut new_state = current.clone();
                new_state.push('(');
                stack.push((new_state, open + 1, close));
            }
            if close < open {
                let mut new_state = current.clone();
                new_state.push(')');
                stack.push((new_state, open, close + 1));
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}