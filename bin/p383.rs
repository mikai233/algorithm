struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0i32; 26];
        for b in magazine.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        for b in ransom_note.bytes() {
            let idx = (b - b'a') as usize;
            cnt[idx] -= 1;
            if cnt[idx] < 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let res = Solution::can_construct("a".to_string(), "b".to_string());
    println!("{res}");
    let res = Solution::can_construct("aa".to_string(), "aab".to_string());
    println!("{res}");
}
