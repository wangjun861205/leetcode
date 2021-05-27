struct Solution;

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; t.len()];
        let mut stack: Vec<usize> = Vec::new();
        for (i, v) in t.iter().enumerate() {
            while stack.len() > 0 && *v > t[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                ans[idx] = (i - idx) as i32;
            }
            stack.push(i);
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
