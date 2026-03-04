use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }
        let word_len = words[0].len();
        let mut word_count = HashMap::with_capacity(words.len());
        for world in &words {
            *word_count.entry(world.as_str()).or_insert(0) += 1;
        }
        let mut result = vec![];
        for offset in 0..word_len {
            let mut left = offset;
            let mut window_count = HashMap::new();
            let mut window_size = 0;
            let limit = s.len().saturating_sub(word_len);
            for right in (offset..=limit).step_by(word_len) {
                if s.len() < right + word_len {
                    break;
                }
                let word = &s[right..right + word_len];
                match word_count.get(word) {
                    None => {
                        window_count.clear();
                        window_size = 0;
                        left = right + word_len;
                    }
                    Some(expected_count) => {
                        let expected_count = *expected_count;
                        let mut cnt = window_count.entry(word).or_insert(0);
                        *cnt += 1;
                        window_size += 1;
                        while *cnt > expected_count {
                            let left_word = &s[left..left + word_len];
                            *window_count.get_mut(left_word).unwrap() -= 1;
                            window_size -= 1;
                            left += word_len;
                            cnt = window_count.get_mut(word).unwrap();
                        }
                        if window_size == words.len() {
                            result.push(left as i32);
                        }
                    }
                }
            }
        }
        result
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec!["foo".to_string(), "bar".to_string()]
        )
    );
    let s = String::from("lingmindraboofooowingdingbarrwingmonkeypoundcake");
    let worlds = vec!["fooo", "barr", "wing", "ding", "wing"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    println!("{:?}", Solution::find_substring(s, worlds));
    println!(
        "{:?}",
        Solution::find_substring("mississippi".to_string(), vec!["mississippis".to_string()])
    );
}
