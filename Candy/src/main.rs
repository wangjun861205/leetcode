struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut l2r = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                l2r[i] = l2r[i - 1] + 1;
            }
        }
        let mut r2l = vec![1; ratings.len()];
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                r2l[i] = r2l[i + 1] + 1;
            }
        }
        l2r.into_iter().zip(r2l).map(|(x, y)| x.max(y)).sum()
    }
}
fn main() {
    println!("{}", Solution::candy(vec![1, 2, 2]));
}
