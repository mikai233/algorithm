struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first_str = strs[0].as_bytes();
        let mut prefix = String::new();

        for (i, &byte) in first_str.iter().enumerate() {
            for s in &strs[1..] {
                if i >= s.len() || s.as_bytes()[i] != byte {
                    return prefix;
                }
            }
            prefix.push(byte as char);
        }
        prefix
    }
}

fn main() {
    println!("{}", Solution::longest_common_prefix(vec!["flower".to_string(), "flowle".to_string(), "flight".to_string()]));
    println!("{}", Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]));
}