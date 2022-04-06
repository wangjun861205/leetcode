struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut counts = vec![0i64; 1024];
        counts[0] = 1;
        let mut mask = 0;
        let mut ans = 0;
        for c in word.chars() {
            mask ^= 1 << (c as usize - 97);
            ans += counts[mask];
            for i in 0..10 {
                ans += counts[mask ^ (1 << i)];
            }
            counts[mask] += 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::wonderful_substrings("aabb".into()));
}
