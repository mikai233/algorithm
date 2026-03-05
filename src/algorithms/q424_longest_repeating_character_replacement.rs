use crate::common::solution::Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut cnt = [0; 26];
        let mut left = 0;
        let mut max_freq = 0;
        let mut best = 0;
        for (right, b) in bytes.iter().enumerate() {
            let idx = (*b - b'A') as usize;
            cnt[idx] += 1;
            max_freq = max_freq.max(cnt[idx]);
            while (right - left + 1) - max_freq > k as usize {
                let left_idx = (bytes[left] - b'A') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            best = best.max(right - left + 1);
        }
        best as i32
    }
}
