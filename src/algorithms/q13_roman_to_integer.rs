use crate::common::solution::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;
        fn value(symbol: char) -> i32 {
            match symbol {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }
        for symbol in s.chars().rev() {
            let curr = value(symbol);
            if curr < prev {
                result -= curr;
            } else {
                result += curr;
                prev = curr;
            }
        }
        result
    }
}
