struct Solution;

impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        if num1.len() < num2.len() {
            num1.insert_str(0, &str::repeat("0", num2.len() - num1.len()));
        } else if num1.len() > num2.len() {
            num2.insert_str(0, &str::repeat("0", num1.len() - num2.len()));
        }
        let mut high_order = 0;
        let mut ans: Vec<char> = num1
            .chars()
            .rev()
            .zip(num2.chars().rev())
            .map(|(c1, c2)| {
                let n1: i32 = c1.to_string().parse().unwrap();
                let n2: i32 = c2.to_string().parse().unwrap();
                let total = n1 + n2 + high_order;
                let cur = total % 10;
                high_order = total / 10;
                cur.to_string().chars().nth(0).unwrap()
            })
            .collect();
        if high_order > 0 {
            ans.push(high_order.to_string().chars().nth(0).unwrap());
        }
        ans.into_iter().rev().collect()
    }
}
fn main() {
    println!("Hello, world!");
}
