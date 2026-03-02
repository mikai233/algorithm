struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut best = nums[0];
        for &ele in &nums[1..] {
            if ele.abs() < best.abs() || (ele.abs() == best.abs() && ele > best) {
                best = ele;
            }
        }
        best
    }
}

fn main() {
    let result = Solution::find_closest_number(vec![-4, -2, 1, 4, 8]);
    println!("{result}");
    let result = Solution::find_closest_number(vec![2, -1, 1]);
    println!("{result}");
}
