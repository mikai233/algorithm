use crate::common::solution::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for n in nums {
            if seen.contains(&n) {
                return true;
            }
            seen.insert(n);
        }
        false
    }
}
