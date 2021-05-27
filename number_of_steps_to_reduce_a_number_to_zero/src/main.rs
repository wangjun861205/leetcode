struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let s = format!("{:b}", num);
        s.chars()
            .skip(1)
            .map(|c| if c == '1' { 2 } else { 1 })
            .sum::<i32>()
            + 1
    }
}
fn main() {
    println!("{}", Solution::number_of_steps(8))
}
