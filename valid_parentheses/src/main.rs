struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                ')' => match stack.last() {
                    None => return false,
                    Some(v) => {
                        if v != &'(' {
                            return false;
                        }
                        stack.pop();
                    }
                },
                ']' => match stack.last() {
                    None => return false,
                    Some(v) => {
                        if v != &'[' {
                            return false;
                        }
                        stack.pop();
                    }
                },
                '}' => match stack.last() {
                    None => return false,
                    Some(v) => {
                        if v != &'{' {
                            return false;
                        }
                        stack.pop();
                    }
                },
                _ => {}
            }
        }
        stack.len() == 0
    }
}
fn main() {
    println!("Hello, world!");
}
