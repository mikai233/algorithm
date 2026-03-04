struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len = 0;
        let mut stack = vec![-1];
        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if let Some(&last) = stack.last() {
                    max_len = max_len.max(i as i32 - last);
                } else {
                    stack.push(i as i32);
                }
            }
        }
        max_len
    }
}

fn main() {
    println!("{}", Solution::longest_valid_parentheses("()".to_string()));
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("()())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("(()"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from(")()())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from(")()())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("()((())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("()(()(()((()())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("()(()(()((()())"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("()()))()"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("(()(((()"))
    );
}
