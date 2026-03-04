use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        let mut max_len = 0;
        let mut left = 0;
        for (right, ch) in s.chars().enumerate() {
            if let Some(last_index) = last_seen.get(&ch)
                && *last_index >= left
            {
                left = *last_index + 1;
            }
            last_seen.insert(ch, right);
            max_len = max(max_len, right - left + 1);
        }
        max_len as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_longest_substring(" ".to_string()));
    println!(
        "{}",
        Solution::length_of_longest_substring("  ".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("pww kew".to_string())
    );
}
