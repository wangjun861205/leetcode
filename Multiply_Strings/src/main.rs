struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_owned();
        }
        let mut step = 0;
        let mut ans: Vec<i32> = vec![0; num1.len() * num2.len() + 1];
        let digits1: Vec<i32> = num1
            .chars()
            .map(|v| v.to_string().parse().unwrap())
            .rev()
            .collect();
        let digits2: Vec<i32> = num2
            .chars()
            .map(|v| v.to_string().parse().unwrap())
            .rev()
            .collect();
        for i in 0..digits1.len() {
            for j in 0..digits2.len() {
                let d1 = digits1[i];
                let d2 = digits2[j];
                let mul = d1 * d2;
                let ori = ans[i + j];
                ans[i + j] = ((mul % 10) + step + ori) % 10;
                step = mul / 10 + ((mul % 10) + step + ori) / 10;
            }
            if step != 0 {
                ans[i + digits2.len()] += step;
            }
            step = 0;
        }
        if step != 0 {
            *ans.last_mut().unwrap() += step;
        }

        let mut s: String = ans.into_iter().map(|v| format!("{}", v)).rev().collect();
        s = s.trim_start_matches('0').to_owned();
        s
    }
}
fn main() {
    println!("{}", Solution::multiply("9".to_owned(), "9".to_owned()));
}
