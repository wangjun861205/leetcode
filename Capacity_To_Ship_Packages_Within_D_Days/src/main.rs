struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut hi: i32 = weights.iter().sum();
        let mut lo: i32 = *weights.iter().max().unwrap();
        'outer: while lo < hi {
            let cap = lo + (hi - lo) / 2;
            let mut d = 1;
            let mut sum = 0;
            for w in &weights {
                sum += *w;
                if sum > cap {
                    d += 1;
                    sum = *w;
                }
                if d > days {
                    lo = cap + 1;
                    continue 'outer;
                }
            }
            hi = cap;
        }
        lo
    }
}
fn main() {
    println!("{}", Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4));
}
