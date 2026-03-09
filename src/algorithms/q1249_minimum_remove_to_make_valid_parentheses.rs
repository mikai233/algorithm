use crate::common::solution::Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let chars = s.chars().collect::<Vec<_>>();
        let mut tmp = vec![];
        let mut open = 0;
        for ch in chars {
            match ch {
                '(' => {
                    open += 1;
                    tmp.push(ch);
                }
                ')' => {
                    if open > 0 {
                        open -= 1;
                        tmp.push(ch);
                    }
                }
                _ => {
                    tmp.push(ch);
                }
            }
        }
        let mut res = Vec::with_capacity(tmp.len());
        for ch in tmp.into_iter().rev() {
            if ch == '(' && open > 0 {
                open -= 1;
            } else {
                res.push(ch);
            }
        }
        res.reverse();
        res.into_iter().collect()
    }
}
