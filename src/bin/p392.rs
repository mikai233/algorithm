struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut iter = s.chars();
        let mut need = iter.next();
        for ele in t.chars() {
            if Some(ele) == need {
                need = iter.next();
                if need.is_none() {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {}
