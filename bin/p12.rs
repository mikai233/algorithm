struct Solution;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut roman = String::new();
        'o: while num > 0 {
            for (r, i) in symbols {
                if num >= i {
                    num -= i;
                    roman.push_str(r);
                    continue 'o;
                }
            }
        }
        roman
    }
}
fn main() {
    assert_eq!(Solution::int_to_roman(3), "III");
}
