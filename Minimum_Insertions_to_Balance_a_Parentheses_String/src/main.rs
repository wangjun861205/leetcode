struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut ans = 0;
        let mut right = 0;
        for c in s.chars() {
            if c == ')' {
                right -= 1;
                if right < 0 {
                    ans += 1;
                    right += 2;
                }
                continue;
            }
            if right % 2 == 1 {
                ans += 1;
                right -= 1;
            }
            right += 2;
        }
        ans += right;
        ans
    }
}

fn main() {
    println!("{}", Solution::min_insertions("))())(".into()));
}
