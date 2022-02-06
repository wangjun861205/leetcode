struct Solution;

impl Solution {
    pub fn minimum_buckets(street: String) -> i32 {
        let mut state = "";
        let mut ans = 0;
        for c in street.chars() {
            if c == 'H' {
                if state == "" {
                    state = "H";
                    continue;
                }
                if state == "H" {
                    return -1;
                }
                match state {
                    "." => {
                        state = ".H";
                        ans += 1;
                    }
                    ".H" => {
                        state = "H";
                    }
                    "H." => {
                        state = "";
                    }
                    _ => unimplemented!(),
                }
            } else {
                if state == "" {
                    state = ".";
                }
                match state {
                    "H" => {
                        state = "H.";
                        ans += 1
                    }
                    "." => (),
                    "H." => state = ".",
                    ".H" => state = "H.",
                    _ => unimplemented!(),
                }
            }
        }
        if state == "H" {
            return -1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::minimum_buckets(".HHH.".into()));
}
