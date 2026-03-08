use crate::common::solution::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        if s.len() < p.len() {
            return res;
        }
        let mut need = [0; 26];
        for &b in p.as_bytes() {
            let index = (b - b'a') as usize;
            need[index] += 1;
        }
        let mut window = [0; 26];
        let m = p.len();
        let s_bytes = s.as_bytes();
        for (i, &b) in s_bytes.iter().enumerate() {
            window[(b - b'a') as usize] += 1;
            if i >= m {
                let index = (s_bytes[i - m] - b'a') as usize;
                window[index] -= 1;
            }
            if need == window {
                res.push((i - m + 1) as i32);
            }
        }
        res
    }
}
