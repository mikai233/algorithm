use crate::common::solution::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result = std::collections::HashMap::new();
        for ele in strs {
            let mut cnt = [0; 26];
            for ch in ele.bytes() {
                let index = (ch - b'a') as usize;
                cnt[index] += 1;
            }
            result.entry(cnt).or_insert(vec![]).push(ele);
        }
        result.into_values().collect()
    }
}
