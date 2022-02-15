struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut ans = 0;
        for l in left {
            ans = ans.max(l);
        }
        for r in right {
            ans = ans.max(n - r);
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
