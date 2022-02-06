struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut ans = 0;
        let mut count = 1;
        let mut prev = None;
        for c in s.chars() {
            if let Some(p) = prev {
                if c == p {
                    count += 1;
                } else {
                    ans = ans.max(count);
                    count = 1;
                    prev = Some(c);
                }
            } else {
                count = 1;
                prev = Some(c)
            }
        }
        ans = ans.max(count);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
