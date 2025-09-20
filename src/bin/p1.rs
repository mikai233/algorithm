use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited: HashMap<i32, usize> = HashMap::new();
        let mut result = vec![];
        for (index, x) in nums.iter().enumerate() {
            match visited.get(&(target - x)) {
                None => {
                    visited.insert(*x, index);
                }
                Some(another_index) => {
                    result.push(index as i32);
                    result.push(*another_index as i32);
                    break;
                }
            }
        }
        result
    }
}
fn main() {
    assert_eq!(
        Solution::two_sum(vec![2, 7, 11, 15], 9)
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![0, 1].into_iter().collect::<HashSet<_>>()
    );
}
