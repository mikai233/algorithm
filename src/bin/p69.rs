struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let (mut left, mut right) = (1, x / 2 + 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let sq = mid as i64 * mid as i64;

            if sq == x as i64 {
                return mid;
            } else if sq < x as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right
    }
}

fn main() {
    println!("{}", Solution::my_sqrt(2147395599));
}
