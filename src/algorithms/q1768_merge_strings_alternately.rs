use crate::common::solution::Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = String::with_capacity(word1.len() + word2.len());
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(ch1), Some(ch2)) => {
                    ans.push(ch1);
                    ans.push(ch2);
                }
                (Some(ch), None) => {
                    ans.push(ch);
                    ans.extend(iter1);
                    break;
                }
                (None, Some(ch)) => {
                    ans.push(ch);
                    ans.extend(iter2);
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
        ans
    }
}
