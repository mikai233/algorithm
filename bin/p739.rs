struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut st = vec![];
        for (i, temperature) in temperatures.iter().enumerate() {
            while let Some(&j) = st.last()
                && *temperature > temperatures[j]
            {
                st.pop();
                ans[j] = (i - j) as i32;
            }
            st.push(i);
        }
        ans
    }
}

fn main() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let ans = Solution::daily_temperatures(temperatures);
    println!("{ans:?}");
}
