struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        let mut result = String::with_capacity(word1.len() + word2.len());
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c), None) | (None, Some(c)) => {
                    result.push(c);
                }
                _ => break,
            }
        }
        result
    }
}

fn main() {
    let mut word1 = "abc";
    let mut word2 = "pqr";
    let result = Solution::merge_alternately(word1.to_string(), word2.to_string());
    println!("{result}");
    let mut word1 = "ab";
    let mut word2 = "pqr";
    let result = Solution::merge_alternately(word1.to_string(), word2.to_string());
    println!("{result}");
    let mut word1 = "abcde";
    let mut word2 = "pqr";
    let result = Solution::merge_alternately(word1.to_string(), word2.to_string());
    println!("{result}");
}
