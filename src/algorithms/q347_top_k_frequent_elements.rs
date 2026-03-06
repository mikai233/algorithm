use crate::common::solution::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut frequence = std::collections::HashMap::new();
        for n in nums {
            *frequence.entry(n).or_insert(0) += 1;
        }
        let mut heap = std::collections::BinaryHeap::new();
        for (num, count) in frequence.into_iter() {
            heap.push(std::cmp::Reverse((count, num)));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter()
            .map(|std::cmp::Reverse((_, num))| num)
            .collect()
    }
}
