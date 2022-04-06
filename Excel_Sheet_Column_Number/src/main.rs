struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans = 0;
        for (i, c) in column_title.chars().rev().enumerate() {
            ans += (c as i32 - 64) * 26_i32.pow(i as u32);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::title_to_number("A".into()));
}
