struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut k = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[k] {
                k += 1;
                nums[k] = nums[i];
            }
        }
        (k + 1) as i32
    }
}
fn main() {
    println!("{}", Solution::remove_duplicates(&mut vec![1, 1, 2]));
    println!("{}", Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
    println!("{}", Solution::remove_duplicates(&mut vec![0, 0, 0]));
}