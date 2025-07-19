use std::cmp::max;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = HashSet::new();
        let mut max_len = 0;
        let mut left = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for (right, ch) in chars.iter().enumerate() {
            while seen.contains(ch) {
                seen.remove(&chars[left]);
                left += 1;
            }
            seen.insert(ch);
            max_len = max(max_len, right - left + 1);
        }
        max_len as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_longest_substring(" ".to_string()));
    println!("{}", Solution::length_of_longest_substring("abcabcbb".to_string()));
    println!("{}", Solution::length_of_longest_substring("bbbbb".to_string()));
    println!("{}", Solution::length_of_longest_substring("pwwkew".to_string()));
    println!("{}", Solution::length_of_longest_substring("pww kew".to_string()));
}