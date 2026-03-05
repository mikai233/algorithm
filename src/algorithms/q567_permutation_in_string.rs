use crate::common::solution::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (n1, n2) = (s1.len(), s2.len());
        if n1 > n2 {
            return false;
        }
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let mut need = [0; 26];
        let mut window = [0; 26];
        for &b in b1 {
            let index = (b - b'a') as usize;
            need[index] += 1;
        }
        for &b in &b2[..n1] {
            let index = (b - b'a') as usize;
            window[index] += 1;
        }
        if need == window {
            return true;
        }
        for i in n1..n2 {
            let add_idx = (b2[i] - b'a') as usize;
            let rem_idx = (b2[i - n1] - b'a') as usize;
            window[add_idx] += 1;
            window[rem_idx] -= 1;
            if window == need {
                return true;
            }
        }
        false
    }
}
