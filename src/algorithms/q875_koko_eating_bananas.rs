use crate::common::solution::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        fn hours_needed(piles: &Vec<i32>, k: i64) -> i64 {
            piles.iter().map(|&p| (p as i64 + k - 1) / k).sum()
        }
        let (mut l, mut r) = (1, *piles.iter().max().unwrap());
        while l < r {
            let mid = l + (r - l) / 2;
            if hours_needed(&piles, mid as i64) <= h as i64 {
                r = mid;
            } else {
                l = mid + 1
            }
        }
        l
    }
}
