struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut trusted_count = vec![0; n as usize];
        let mut trust_count = vec![0; n as usize];
        for t in &trust {
            trusted_count[t[1] as usize - 1] += 1;
            trust_count[t[0] as usize - 1] += 1;
        }
        for t in &trust {
            if trusted_count[t[1] as usize - 1] == n - 1 && trust_count[t[1] as usize - 1] == 0 {
                return t[1];
            }
        }
        -1
    }
}
fn main() {
    println!("{}", Solution::find_judge(2, vec![vec![1, 2]]));
}
