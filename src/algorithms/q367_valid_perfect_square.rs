use crate::common::solution::Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        if num < 1 {
            return false;
        }
        let (mut l, mut r) = (1, num / 2 + 2);
        while l < r {
            let mid = l + (r - l) / 2;
            match (mid * mid).cmp(&num) {
                std::cmp::Ordering::Less => {
                    l = mid + 1;
                }
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Greater => {
                    r = mid;
                }
            }
        }
        false
    }
}
