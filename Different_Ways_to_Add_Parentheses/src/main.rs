struct Solution;

impl Solution {
    fn eval(tokens: Vec<String>) -> Vec<i32> {
        if tokens.len() == 1 {
            return vec![tokens[0].parse().unwrap()];
        }
        let mut ans = Vec::new();
        for i in 0..tokens.len() {
            match tokens[i].as_str() {
                "+" => {
                    let left_vals = Solution::eval(tokens[..i].to_vec());
                    let right_vals = Solution::eval(tokens[i + 1..].to_vec());
                    for &lv in &left_vals {
                        for &rv in &right_vals {
                            ans.push(lv + rv);
                        }
                    }
                }
                "-" => {
                    let left_vals = Solution::eval(tokens[..i].to_vec());
                    let right_vals = Solution::eval(tokens[i + 1..].to_vec());
                    for &lv in &left_vals {
                        for &rv in &right_vals {
                            ans.push(lv - rv);
                        }
                    }
                }
                "*" => {
                    let left_vals = Solution::eval(tokens[..i].to_vec());
                    let right_vals = Solution::eval(tokens[i + 1..].to_vec());
                    for &lv in &left_vals {
                        for &rv in &right_vals {
                            ans.push(lv * rv);
                        }
                    }
                }
                _ => {}
            }
        }
        return ans;
    }
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let tokens = expression.chars().fold(Vec::new(), |mut l, v| {
            match v {
                '+' | '-' | '*' => {
                    l.push(v.to_string());
                }
                _ => {
                    if l.is_empty() {
                        l.push(v.to_string());
                    } else {
                        let last = l.last_mut().unwrap();
                        if last != "+" && last != "-" && last != "*" {
                            l.last_mut().unwrap().push(v);
                        } else {
                            l.push(v.to_string());
                        }
                    }
                }
            }
            l
        });
        Solution::eval(tokens)
    }
}

fn main() {
    println!("Hello, world!");
}
