struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut curr, mut best) = (nums[0], nums[0]);
        for ele in nums.iter().skip(1) {
            curr = std::cmp::max(*ele, *ele + curr);
            best = std::cmp::max(best, curr);
        }
        best
    }
}

fn main() {
    let result = Solution::max_sub_array(vec![5, 4, -1, 7, 8]);
    println!("{result}");
}
