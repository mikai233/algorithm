use crate::common::solution::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        for b in magazine.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        for b in ransom_note.bytes() {
            let i = (b - b'a') as usize;
            cnt[i] -= 1;
            if cnt[i] < 0 {
                return false;
            }
        }
        true
    }
}
