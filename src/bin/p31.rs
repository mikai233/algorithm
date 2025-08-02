struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(pivot) = (0..nums.len() - 1).rev().find(|&i| nums[i] < nums[i + 1]) {
            let successor = nums[pivot + 1..].iter().rposition(|&x| x > nums[pivot]).unwrap();
            nums.swap(pivot, pivot + 1 + successor);
            nums[pivot + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

fn main() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
    let mut v = vec![1, 3, 2, 4];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
}