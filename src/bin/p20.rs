struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
                _ => unreachable!()
            }
        }
        stack.is_empty()
    }
}

fn main() {
    println!("{}", Solution::is_valid("()".to_string()));
    println!("{}", Solution::is_valid("({}[])".to_string()));
    println!("{}", Solution::is_valid("({}[]()".to_string()));
    println!("{}", Solution::is_valid("((".to_string()));
}