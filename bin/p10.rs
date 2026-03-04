struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let pattern = p.as_bytes();
        let text = s.as_bytes();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                let p = pattern[j - 1];
                if p != b'*' {
                    if i > 0 && (text[i - 1] == p || p == b'.') {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                } else if j > 1 {
                    let p = pattern[j - 2];
                    if i > 0 && (p == text[i - 1] || p == b'.') {
                        dp[i][j] = dp[i - 1][j];
                    }
                    dp[i][j] |= dp[i][j - 2];
                }
            }
        }
        dp[m][n]
    }
}
fn main() {
    println!("{}", Solution::is_match("aa".to_string(), "a".to_string()));
    println!("{}", Solution::is_match("aa".to_string(), "a*".to_string()));
    println!("{}", Solution::is_match("ab".to_string(), ".*".to_string()));
}
