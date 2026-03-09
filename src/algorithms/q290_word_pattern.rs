use crate::common::solution::Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<_> = s.split_whitespace().collect();
        let chars: Vec<_> = pattern.chars().collect();
        if words.len() != chars.len() {
            return false;
        }
        let mut c2w = std::collections::HashMap::with_capacity(words.len());
        let mut w2c = std::collections::HashMap::with_capacity(words.len());
        for i in 0..words.len() {
            let c = chars[i];
            let w = words[i];
            match c2w.entry(c) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    if o.get() != &w {
                        return false;
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(w);
                }
            }
            match w2c.entry(w) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    if o.get() != &c {
                        return false;
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(c);
                }
            }
        }
        true
    }
}
