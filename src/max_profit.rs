use std::slice::range;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_price = prices[0];
    let mut profit = 0;

    for v in prices.iter() {
        if v < &buy_price {
            buy_price = v;
        } else if v - &buy_price > &profit{
            profit = v- buy_price;
        }
    }

    profit
}