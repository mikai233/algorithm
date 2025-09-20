struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        for &b in s.as_bytes().iter().rev() {
            if b == b' ' {
                if len > 0 {
                    break;
                }
            } else {
                len += 1;
            }
        }
        len
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_last_word("Hello World".to_string())
    );
    println!(
        "{}",
        Solution::length_of_last_word("   fly me   to   the moon  ".to_string())
    );
    println!(
        "{}",
        Solution::length_of_last_word("luffy is still joyboy".to_string())
    );
}
