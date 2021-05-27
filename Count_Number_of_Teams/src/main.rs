struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;
        for (i, v) in rating.iter().enumerate() {
            let (mut llc, mut rgc, mut lgc, mut rlc) = (0, 0, 0, 0);
            for l in rating[..i].into_iter() {
                if l < v {
                    llc += 1;
                } else {
                    lgc += 1;
                }
            }
            for r in rating[i + 1..].into_iter() {
                if r < v {
                    rlc += 1;
                } else {
                    rgc += 1;
                }
            }
            count += llc * rgc + lgc * rlc;
        }
        count
    }
}
fn main() {
    println!("{}", Solution::num_teams(vec![2, 5, 3, 4, 1]));
}
