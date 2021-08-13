struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let max = 10000;
        let mut dists = vec![vec![max; mat[0].len()]; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    dists[i][j] = 0;
                }
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..mat.len() {
                for j in 0..mat[0].len() {
                    let left = if j > 0 { dists[i][j - 1] } else { max };
                    let right = if j < mat[0].len() - 1 { dists[i][j + 1] } else { max };
                    let top = if i > 0 { dists[i - 1][j] } else { max };
                    let bottom = if i < mat.len() - 1 { dists[i + 1][j] } else { max };
                    let min = left.min(right.min(top.min(bottom))) + 1;
                    if dists[i][j] > min {
                        dists[i][j] = min;
                        changed = true
                    }
                }
            }
        }
        dists
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::update_matrix(vec![
            vec![1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 0, 1, 1, 1, 1]
        ])
    );
}
