struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans: i64 = 0;
        let mut length: i64 = 0;
        for c in chars {
            if c == '1' {
                length += 1;
                ans += length;
            } else {
                length = 0;
            }
        }
        (ans % (1e9 as i64 + 7)) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
