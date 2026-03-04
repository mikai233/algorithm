struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
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
    println!("{}", Solution::remove_duplicates(&mut [1, 1, 2]));
    println!(
        "{}",
        Solution::remove_duplicates(&mut [0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    );
    println!("{}", Solution::remove_duplicates(&mut [0, 0, 0]));
}
