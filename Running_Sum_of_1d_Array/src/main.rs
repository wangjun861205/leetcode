struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
