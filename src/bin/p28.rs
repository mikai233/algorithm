struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let haystack_len = haystack.len();
        let needle_len = needle.len();
        if needle_len > haystack_len {
            return -1;
        }
        for i in 0..=haystack_len - needle_len {
            if haystack[i..i + needle_len] == needle {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::str_str(String::from("sadbutsad"), String::from("sad")));
    println!("{}", Solution::str_str(String::from("leetcode"), String::from("leeto")));
    println!("{}", Solution::str_str(String::from("leeleetocode"), String::from("leeto")));
    println!("{}", Solution::str_str(String::from("aaa"), String::from("aaaa")));
}