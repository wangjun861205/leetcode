struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        match s.len() {
            0 => return 0,
            1 => {
                if s == "0" {
                    return 0;
                } else {
                    return 1;
                }
            }
            _ => {
                let first_count =
                    Solution::num_decodings(String::from_utf8(s.as_bytes()[1..].to_vec()).unwrap());
                let second_count =
                    Solution::num_decodings(String::from_utf8(s.as_bytes()[2..].to_vec()).unwrap());
                // let val = String::from_utf8(s.as_bytes()[0..2].to_vec())
                //     .unwrap()
                //     .parse::<i32>()
                //     .unwrap();
                return first_count + second_count;
                // if val == 0 {
                //     return first_count + second_count;
                // } else if val <= 10 || val == 20 {
                //     return first_count + second_count;
                // } else if val <= 26 {
                //     return first_count + second_count + 1;
                // } else {
                //     return first_count + second_count;
                // }
            }
        }
    }
}

fn main() {
    println!("{}", Solution::num_decodings("226".to_owned()));
}
