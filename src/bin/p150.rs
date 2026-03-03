struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut st = vec![];
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let right = st.pop().unwrap();
                    let left = st.pop().unwrap();
                    st.push((left + right));
                }
                "-" => {
                    let right = st.pop().unwrap();
                    let left = st.pop().unwrap();
                    st.push((left - right));
                }
                "*" => {
                    let right = st.pop().unwrap();
                    let left = st.pop().unwrap();
                    st.push((left * right));
                }
                "/" => {
                    let right = st.pop().unwrap();
                    let left = st.pop().unwrap();
                    st.push((left / right));
                }
                num => {
                    let num = num.parse::<i32>().unwrap();
                    st.push(num);
                }
            }
        }
        *st.last().unwrap()
    }
}

fn main() {
    todo!();
}
