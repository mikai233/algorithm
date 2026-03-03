struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut l = 0i32;
        let mut r = s.len() as i32 - 1;
        while l < r {
            while l < r && !bytes[l as usize].is_ascii_alphanumeric() {
                l += 1;
            }
            while l < r && !bytes[r as usize].is_ascii_alphanumeric() {
                r -= 1;
            }
            if l > r {
                break;
            }
            let a = bytes[l as usize].to_ascii_lowercase();
            let b = bytes[r as usize].to_ascii_lowercase();
            if a != b {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

fn main() {
    let m = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
    println!("{m}");
    let m = Solution::is_palindrome(" ".to_string());
    println!("{m}");
    let m = Solution::is_palindrome("a".to_string());
    println!("{m}");
}
