use crate::common::solution::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];
            if diff > 0 {
                profit += diff;
            }
        }
        profit
    }
}
