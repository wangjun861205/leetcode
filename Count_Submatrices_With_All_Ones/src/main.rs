struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for row in 0..mat.len() {
            let mut bits = vec![1; mat[0].len()];
            for r in row..mat.len() {
                for c in 0..mat[0].len() {
                    bits[c] &= mat[r][c];
                }
                let mut count = 0;
                let mut length = 0;
                for b in &bits {
                    if b == &1 {
                        length += 1;
                        count += length;
                    } else {
                        length = 0;
                    }
                }
                ans += count;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
    );
}
