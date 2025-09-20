struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let negative = (dividend < 0) ^ (divisor < 0);
        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let mut quotient = 0;
        while dividend <= divisor {
            let mut power_of_two = -1;
            let mut value = divisor;
            while value >= i32::MIN >> 1 && dividend <= value + value {
                value += value;
                power_of_two += power_of_two;
            }
            dividend -= value;
            quotient += power_of_two;
        }
        if !negative {
            if quotient == i32::MIN {
                i32::MAX
            } else {
                -quotient
            }
        } else {
            quotient
        }
    }
}

fn main() {
    println!("{}", Solution::divide(10, 3));
    println!("{}", Solution::divide(7, -3));
    println!("{}", Solution::divide(-7, -3));
}
