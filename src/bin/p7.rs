struct Solution;
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut reversed = 0;
        while x != 0 {
            if reversed > i32::MAX / 10 || reversed < i32::MIN / 10 {
                return 0;
            }
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }
        reversed
    }
}
fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(i32::MAX));
    println!("{}", Solution::reverse(i32::MIN));
    println!("{}", Solution::reverse(10));
    println!("{}", Solution::reverse(0));
}