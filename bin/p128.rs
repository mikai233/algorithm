use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = nums.into_iter().collect::<HashSet<_>>();
        let mut best = 0;
        for ele in &set {
            if !set.contains(&(ele - 1)) {
                let mut curr = *ele;
                let mut curr_best = 0;
                while set.contains(&curr) {
                    curr += 1;
                    curr_best += 1;
                }
                best = curr_best.max(best);
            }
        }
        best
    }
}

fn main() {
    let len = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
    println!("{len}");
}
