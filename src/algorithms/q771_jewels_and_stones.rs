use crate::common::solution::Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut seen = [false; 128];
        for b in jewels.bytes() {
            seen[b as usize] = true;
        }
        let mut count = 0;
        for b in stones.bytes() {
            if seen[b as usize] {
                count += 1;
            }
        }
        count
    }
}
