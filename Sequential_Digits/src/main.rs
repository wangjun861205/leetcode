struct Solution;

impl Solution {
    pub fn sequential_digits(mut low: i32, high: i32) -> Vec<i32> {
        let candidates = vec![
            12, 123, 1234, 12345, 123456, 1234567, 12345678, 123456789, 23, 234, 2345, 23456,
            234567, 2345678, 23456789, 34, 345, 3456, 34567, 345678, 3456789, 45, 456, 4567, 45678,
            456789, 56, 567, 5678, 56789, 67, 678, 6789, 78, 789, 89,
        ];
        let mut ans: Vec<i32> = candidates
            .into_iter()
            .filter(|v| v >= &low && v <= &high)
            .collect();
        ans.sort();
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
