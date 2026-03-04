use crate::common::solution::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = s.chars();
        let mut next = iter.next();
        for ch in t.chars() {
            if Some(ch) == next {
                next = iter.next();
                if next.is_none() {
                    break;
                }
            }
        }
        next.is_none()
    }
}
