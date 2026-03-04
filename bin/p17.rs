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

    pub fn mapping(number: i32) -> &'static [&'static str] {
        static MAPPING: [&[&str]; 10] = [
            &[],
            &[],
            &["a", "b", "c"],
            &["d", "e", "f"],
            &["g", "h", "i"],
            &["j", "k", "l"],
            &["m", "n", "o"],
            &["p", "q", "r", "s"],
            &["t", "u", "v"],
            &["w", "x", "y", "z"],
        ];
        MAPPING[number as usize]
    }
}
fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}
