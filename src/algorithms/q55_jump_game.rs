use crate::common::solution::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;
        let last_index = nums.len() - 1;
        for (position, &max_jump) in nums.iter().enumerate() {
            if position > max_reachable {
                return false;
            }
            max_reachable = max_reachable.max(position + max_jump as usize);
            if max_reachable >= last_index {
                return true;
            }
        }
        true
    }
}
