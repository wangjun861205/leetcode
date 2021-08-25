struct Solution;

impl Solution {
    fn to_complex(num: String) -> (i32, i32) {
        let mut rs = String::new();
        let mut is = String::new();
        let mut real_complete = false;
        for c in num.chars() {
            if c.is_numeric() || c == '-' {
                if real_complete {
                    is.push(c);
                } else {
                    rs.push(c);
                }
            } else if c == '+' {
                real_complete = true;
            }
        }
        let r = rs.parse::<i32>().unwrap();
        let i = is.parse::<i32>().unwrap();
        (r, i)
    }

    fn to_str(cpx: (i32, i32)) -> String {
        format!("{}+{}i", cpx.0, cpx.1)
    }
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (r1, i1) = Solution::to_complex(num1);
        let (r2, i2) = Solution::to_complex(num2);
        Solution::to_str((r1 * r2 - i1 * i2, r1 * i2 + r2 * i1))
    }
}
fn main() {
    println!(
        "{}",
        Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned())
    )
}
