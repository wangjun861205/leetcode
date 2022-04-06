struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            let temp = s[i];
            s[i] = s[j];
            s[j] = temp;
            i += 1;
            j -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
