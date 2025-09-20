struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut lower, mut upper) = (0, 0);
        let s_bytes = s.as_bytes();
        for i in 0..s_bytes.len() {
            let mut c_lower = i;
            let mut c_upper = i;
            Self::expand_around(s_bytes, &mut c_lower, &mut c_upper);
            if (c_upper - c_lower) > (upper - lower) {
                lower = c_lower;
                upper = c_upper;
            }
            if let Some(n) = &s_bytes.get(i + 1)
                && **n == s_bytes[i] {
                    let mut c_lower = i;
                    let mut c_upper = i + 1;
                    Self::expand_around(s_bytes, &mut c_lower, &mut c_upper);
                    if (c_upper - c_lower) > (upper - lower) {
                        lower = c_lower;
                        upper = c_upper;
                    }
                }
        }
        s[lower..=upper].to_string()
    }

    fn expand_around(s: &[u8], lower: &mut usize, upper: &mut usize) {
        loop {
            if *lower > 0 && *upper < s.len() - 1 && s[*lower - 1] == s[*upper + 1] {
                *lower -= 1;
                *upper += 1;
            } else {
                break;
            }
        }
    }
}
fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
    println!("{}", Solution::longest_palindrome("a".to_string()));
    println!("{}", Solution::longest_palindrome("cbbbd".to_string()));
}
