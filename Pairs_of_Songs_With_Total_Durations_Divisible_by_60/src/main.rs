struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let counts = time.iter().fold(vec![0; 60], |mut l, v| {
            l[(v % 60) as usize] += 1;
            l
        });
        let mut ans = counts[0] * (counts[0] - 1) / 2 + counts[30] * (counts[30] - 1) / 2;
        let mut i = 1_usize;
        let mut j = 59_usize;
        while i < j {
            ans += counts[i] * counts[j];
            i += 1;
            j -= 1;
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
