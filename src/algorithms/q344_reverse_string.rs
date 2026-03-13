use crate::common::solution::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            let [a, b] = s.get_disjoint_mut([i, j]).unwrap();
            std::mem::swap(a, b);
            i += 1;
            j -= 1;
        }
    }
}
