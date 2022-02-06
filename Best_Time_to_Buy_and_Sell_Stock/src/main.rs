struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prev_min = i32::MAX;
        let mut ans = 0;
        for p in prices {
            ans = ans.max(p - prev_min);
            prev_min = prev_min.min(p);
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
