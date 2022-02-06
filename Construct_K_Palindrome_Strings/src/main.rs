struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let counts = s.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let odd_counts = counts.iter().filter(|&c| *c % 2 == 1).count();
        if odd_counts > k as usize {
            return false;
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
