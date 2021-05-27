struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut first_col_zero = false;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if matrix[r][c] == 0 {
                    if c == 0 {
                        first_col_zero = true;
                    } else {
                        matrix[r][0] = 0;
                        matrix[0][c] = 0;
                    }
                }
            }
        }
        for r in 1..matrix.len() {
            for c in 1..matrix[0].len() {
                if matrix[r][0] == 0 || matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }
        if matrix[0][0] == 0 {
            for c in matrix[0].iter_mut() {
                *c = 0;
            }
        }
        if first_col_zero {
            for r in 0..matrix.len() {
                matrix[r][0] = 0
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
