struct Solution;

impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let total: i32 = colsum.iter().sum();
        if upper + lower != total {
            return Vec::new();
        }
        let mut matrix = vec![vec![0; colsum.len()]; 2];
        for (i, sum) in colsum.iter().enumerate() {
            if sum == &2 {
                upper -= 1;
                lower -= 1;
                matrix[0][i] = 1;
                matrix[1][i] = 1;
            }
        }
        if upper < 0 || lower < 0 {
            return Vec::new();
        }
        for (i, sum) in colsum.iter().enumerate() {
            if sum == &1 {
                if upper == 0 && lower == 0 {
                    return Vec::new();
                }
                if upper > 0 {
                    upper -= 1;
                    matrix[0][i] = 1;
                } else {
                    lower -= 1;
                    matrix[1][i] = 1;
                }
            }
        }
        matrix
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_matrix(9, 2, vec![0, 1, 2, 0, 0, 0, 0, 0, 2, 1, 2, 1, 2])
    );
}
