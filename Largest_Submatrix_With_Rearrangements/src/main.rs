struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut cols = matrix[0].len();
        let mut pillars = vec![vec![0; matrix[0].len()]; matrix.len() + 1];
        for i in 1..=matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i - 1][j] == 0 {
                    pillars[i][j] = 0;
                } else {
                    pillars[i][j] = pillars[i - 1][j] + 1;
                }
            }
        }
        let mut ans = 0;
        for mut row in pillars[1..].to_vec() {
            row.sort();
            let mut area = 0;
            for (i, v) in row.into_iter().enumerate() {
                area = area.max(v * (cols - i) as i32);
            }
            ans = ans.max(area);
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]])
    );
}
