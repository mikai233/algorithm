struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut combinations = vec!["".to_string()];
        for digit in digits.chars() {
            let mut new_combinations = vec![];
            for combination in &combinations {
                for x in Self::mapping(digit.to_digit(10).unwrap() as i32) {
                    new_combinations.push(format!("{}{}", combination, x))
                }
            }
            combinations = new_combinations;
        }
        combinations
    }

    pub fn mapping(number: i32) -> Vec<&'static str> {
        match number {
            2 => vec!["a", "b", "c"],
            3 => vec!["d", "e", "f"],
            4 => vec!["g", "h", "i"],
            5 => vec!["j", "k", "l"],
            6 => vec!["m", "n", "o"],
            7 => vec!["p", "q", "r", "s"],
            8 => vec!["t", "u", "v"],
            9 => vec!["w", "x", "y", "z"],
            _ => panic!("Invalid number: {}", number),
        }
    }
}
fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}