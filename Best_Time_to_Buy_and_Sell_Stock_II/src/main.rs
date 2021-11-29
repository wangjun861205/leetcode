struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prev = prices[0];
        let mut benefit = 0;
        for p in prices.into_iter().skip(1) {
            if p > prev {
                benefit += p - prev;
            }
            prev = p;
        }
        benefit
    }
}

fn main() {
    println!("Hello, world!");
}
