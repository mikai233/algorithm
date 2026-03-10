use crate::common::solution::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut ans: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            if ans.is_empty() || ans.last().unwrap()[1] < interval[0] {
                ans.push(interval);
            } else {
                let last = ans.last_mut().unwrap();
                last[1] = last[1].max(interval[1]);
            }
        }
        ans
    }
}
