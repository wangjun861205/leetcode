struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let freq_s = s.chars().fold(vec![0; 26], |mut l, v| {
            l[v as usize - 97] += 1;
            l
        });
        let freq_t = t.chars().fold(vec![0; 26], |mut l, v| {
            l[v as usize - 97] += 1;
            l
        });
        freq_t
            .into_iter()
            .zip(freq_s)
            .map(|(a, b)| if a > b { a - b } else { 0 })
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}
