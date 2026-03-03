struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut st = vec![];
        for operation in operations {
            match operation.as_str() {
                "C" => {
                    st.pop();
                }
                "D" => {
                    let last = st.last().unwrap();
                    st.push(*last * 2);
                }
                "+" => {
                    let n = st.len();
                    let sum = st[n - 1] + st[n - 2];
                    st.push(sum);
                }
                n => {
                    let val = n.parse::<i32>().unwrap();
                    st.push(val);
                }
            }
        }
        st.into_iter().sum()
    }
}

fn main() {
    let sum = Solution::cal_points(
        ["5", "-2", "4", "C", "D", "9", "+", "+"]
            .map(|s| s.to_string())
            .to_vec(),
    );
    println!("{sum}");
}
