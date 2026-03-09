use crate::common::solution::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s_chars: Vec<_> = s.chars().collect();
        let t_chars: Vec<_> = t.chars().collect();
        let mut a2b = std::collections::HashMap::new();
        let mut b2a = std::collections::HashMap::new();
        for i in 0..s.len() {
            let a = s_chars[i];
            let b = t_chars[i];
            match a2b.entry(a) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    if *o.get() != b {
                        return false;
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(b);
                }
            }
            match b2a.entry(b) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    if *o.get() != a {
                        return false;
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(a);
                }
            }
        }
        true
    }
}
