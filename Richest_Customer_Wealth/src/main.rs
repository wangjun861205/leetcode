struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for acc in accounts {
            ans = ans.max(acc.into_iter().sum());
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
