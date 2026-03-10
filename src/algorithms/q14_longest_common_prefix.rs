use crate::common::solution::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s = strs[0].as_bytes();
        for (i, &b) in s.iter().enumerate() {
            for s in strs.iter().skip(1) {
                let bytes = s.as_bytes();
                if bytes.len() <= i || bytes[i] != b {
                    return strs[0].as_str()[0..i].to_string();
                }
            }
        }
        strs[0].clone()
    }
}
