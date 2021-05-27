struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ans: Vec<char> = Vec::new();
        while column_number > 0 {
            ans.insert(0, (65_u8 + ((column_number - 1) % 26) as u8) as char);
            column_number = (column_number - 1) / 26;
        }
        ans.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::convert_to_title(27));
}
