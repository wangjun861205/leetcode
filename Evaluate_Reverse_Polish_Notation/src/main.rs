use std::hint::unreachable_unchecked;

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::new();
        for t in tokens {
            match t.as_str() {
                "+" | "-" | "*" | "/" => {
                    let second: i32 = stack.pop().unwrap().parse().unwrap();
                    let first: i32 = stack.pop().unwrap().parse().unwrap();
                    let ans = match t.as_str() {
                        "+" => first + second,
                        "-" => first - second,
                        "*" => first * second,
                        "/" => first / second,
                        _ => unreachable!(),
                    };
                    stack.push(ans.to_string());
                }
                _ => stack.push(t.to_owned()),
            }
        }
        stack.last().unwrap().parse().unwrap()
    }
}
fn main() {
    println!("Hello, world!");
}
