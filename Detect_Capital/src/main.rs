struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let first_capital = if word.chars().nth(0).unwrap().is_uppercase() {
            true
        } else {
            false
        };
        let mut remain_capital: Option<bool> = None;
        for c in word.chars().skip(1) {
            if c.is_lowercase() {
                if let Some(rc) = remain_capital {
                    if rc {
                        return false;
                    }
                } else {
                    remain_capital = Some(false);
                }
            } else {
                if let Some(rc) = remain_capital {
                    if !rc {
                        return false;
                    }
                } else {
                    remain_capital = Some(true);
                }
            }
        }
        if let Some(rc) = remain_capital {
            if !first_capital && rc {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
