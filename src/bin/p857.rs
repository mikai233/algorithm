struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers = quality
            .iter()
            .zip(wage.iter())
            .map(|(q, w)| (*w as f64 / *q as f64, *q))
            .collect::<Vec<_>>();
        workers.sort_by(|(r1, _), (r2, _)| r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal));

        let mut heap = std::collections::BinaryHeap::new();
        let mut ans = f64::INFINITY;
        let mut sum_quality = 0;

        for (ratio, quality) in workers {
            heap.push(quality);
            sum_quality += quality;
            if heap.len() > k as usize {
                sum_quality -= heap.pop().unwrap_or_default();
            }
            if heap.len() == k as usize {
                let next_ans = ratio * sum_quality as f64;
                if next_ans < ans {
                    ans = next_ans;
                }
            }
        }
        ans
    }
}

fn main() {
    let ans = Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2);
    println!("{ans}");
}
