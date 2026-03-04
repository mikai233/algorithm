use crate::common::solution::Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers = quality
            .into_iter()
            .zip(wage)
            .map(|(q, w)| {
                let r = w as f64 / q as f64;
                (r, q)
            })
            .collect::<Vec<_>>();
        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut heap = std::collections::BinaryHeap::new();
        let mut quality_sum = 0;
        let mut ans = f64::INFINITY;
        for (r, q) in workers {
            heap.push(q);
            quality_sum += q;
            if heap.len() > k as usize {
                quality_sum -= heap.pop().unwrap();
            }
            if heap.len() == k as usize {
                ans = ans.min(r * quality_sum as f64);
            }
        }
        ans
    }
}
