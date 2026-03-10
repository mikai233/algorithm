use crate::common::solution::Solution;

impl Solution {
    pub fn max_profit_121(prices: Vec<i32>) -> i32 {
        let mut buy_price = prices[0];
        let mut max_profit = 0;
        for &price in &prices {
            buy_price = buy_price.min(price);
            max_profit = max_profit.max(price - buy_price);
        }
        max_profit
    }
}
