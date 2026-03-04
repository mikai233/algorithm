struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res: Option<i64> = None;
        let mut positive: Option<bool> = None;
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    let mut r = res.unwrap_or(0);
                    if positive.unwrap_or(true) {
                        r = r * 10 + (c.to_digit(10).unwrap() as i64);
                    } else {
                        r = r * 10 - (c.to_digit(10).unwrap() as i64);
                    }
                    if r > i32::MAX as i64 {
                        return i32::MAX;
                    } else if r < i32::MIN as i64 {
                        return i32::MIN;
                    }
                    res = Some(r);
                }
                '-' => {
                    if res.is_none() && positive.is_none() {
                        positive = Some(false);
                    } else {
                        break;
                    }
                }
                '+' => {
                    if res.is_none() && positive.is_none() {
                        positive = Some(true);
                    } else {
                        break;
                    }
                }
                ' ' => {
                    if res.is_none() && positive.is_none() {
                        continue;
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        res.map(|r| r as i32).unwrap_or(0)
    }
}
fn main() {
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("   -042".to_string()));
    println!("{}", Solution::my_atoi("1337c0d3".to_string()));
    println!("{}", Solution::my_atoi("0-1".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("   +0 123".to_string()));
    println!("{}", Solution::my_atoi("  +  413".to_string()));
}
