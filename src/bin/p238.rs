struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut prefix = 1;
        for (i, n) in nums.iter().enumerate() {
            res[i] *= prefix;
            prefix *= *n;
        }
        let mut suffix = 1;
        for (i, n) in nums.iter().enumerate().rev() {
            res[i] *= suffix;
            suffix *= *n;
        }
        res
    }
}

fn main() {
    let r = Solution::product_except_self(vec![1, 2, 3, 4]);
    println!("{r:?}");
}
