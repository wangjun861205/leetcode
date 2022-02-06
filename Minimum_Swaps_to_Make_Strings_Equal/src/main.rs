struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        if s1.len() != s2.len() {
            return -1;
        }
        let mut x_y = 0;
        let mut y_x = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == 'x' && c2 == 'y' {
                x_y += 1;
            } else if c1 == 'y' && c2 == 'x' {
                y_x += 1;
            }
        }
        if x_y % 2 != y_x % 2 {
            return -1;
        }
        let mut ans = 0;
        ans += x_y / 2 + y_x / 2;
        x_y %= 2;
        y_x %= 2;

        ans += x_y + y_x;
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
