struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut n = 0;
        let mut tmp = num;
        let mut result = String::new();
        while tmp > 0 {
            n += 1;
            tmp /= 10;
        }
        while n > 0 {
            let x = 10i32.pow(n - 1);
            let v = num / x;
            let mut partial_num = x * v;
            num -= partial_num;
            match v {
                4 | 9 => {
                    let (_, romain_high) = Self::get_roman(partial_num).unwrap();
                    let (romain_low, _) = Self::get_roman(Self::roman_to_int(romain_high) - partial_num).unwrap();
                    result.push_str(&format!("{}{}", romain_low, romain_high))
                }
                _ => {
                    if let Some((romain_low, _)) = Self::get_roman(partial_num) {
                        result.push_str(romain_low);
                        partial_num -= Self::roman_to_int(romain_low);
                    }
                    while partial_num > 0 {
                        match Self::get_roman(partial_num) {
                            None => {
                                let (romain_low, _) = Self::get_roman(x).unwrap();
                                partial_num -= Self::roman_to_int(romain_low);
                                result.push_str(romain_low);
                            }
                            Some((romain_low, _)) => {
                                partial_num -= Self::roman_to_int(romain_low);
                                result.push_str(romain_low);
                            }
                        }
                    }
                }
            }
            n -= 1;
        }
        result
    }

    fn get_roman(n: i32) -> Option<(&'static str, &'static str)> {
        match n {
            1..5 => Some(("I", "V")),
            5..10 => Some(("V", "X")),
            10..50 => Some(("X", "L")),
            50..100 => Some(("L", "C")),
            100..500 => Some(("C", "D")),
            500..1000 => Some(("D", "M")),
            1000 => Some(("M", "")),
            _ => None,
        }
    }

    fn roman_to_int(roman: &str) -> i32 {
        match roman {
            "I" => 1,
            "V" => 5,
            "X" => 10,
            "L" => 50,
            "C" => 100,
            "D" => 500,
            "M" => 1000,
            _ => panic!("Invalid romain"),
        }
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(3749));
    println!("{}", Solution::int_to_roman(58));
    println!("{}", Solution::int_to_roman(1994));
    println!("{}", Solution::int_to_roman(4));
    println!("{}", Solution::int_to_roman(3999));
    println!("{}", Solution::int_to_roman(1));
}