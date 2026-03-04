struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut k: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
fn main() {
    println!("{}", Solution::remove_element(&mut [4, 5, 0, -3, 7], 7));
}
