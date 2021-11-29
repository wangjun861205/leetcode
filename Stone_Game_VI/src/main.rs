struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut sums: Vec<(i32, i32, i32)> = alice_values
            .into_iter()
            .zip(bob_values)
            .map(|(a, b)| (a + b, a, b))
            .collect();
        sums.sort_by_key(|v| v.0);
        sums.reverse();
        let mut a = 0;
        let mut b = 0;
        for (i, s) in sums.into_iter().enumerate() {
            if i % 2 == 0 {
                a += s.1;
            } else {
                b += s.2;
            }
        }
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
        return 0;
    }
}
fn main() {
    println!("Hello, world!");
}
