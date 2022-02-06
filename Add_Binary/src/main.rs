struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let chars_a: Vec<char> = a.chars().rev().collect();
        let chars_b: Vec<char> = b.chars().rev().collect();
        let mut ans = String::new();
        let mut extra = false;
        for i in 0..chars_a.len().max(chars_b.len()) {
            let ca = chars_a.get(i).unwrap_or(&'0');
            let cb = chars_b.get(i).unwrap_or(&'0');
            if ca == &'1' && cb == &'1' {
                if extra {
                    ans.insert(0, '1');
                } else {
                    ans.insert(0, '0')
                }
                extra = true;
            } else if ca == &'1' && cb == &'0' || ca == &'0' && cb == &'1' {
                if extra {
                    ans.insert(0, '0');
                } else {
                    ans.insert(0, '1');
                }
            } else {
                if extra {
                    ans.insert(0, '1');
                } else {
                    ans.insert(0, '0');
                }
                extra = false;
            }
        }
        if extra {
            ans.insert(0, '1');
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::add_binary("11".into(), "1".into()));
}
