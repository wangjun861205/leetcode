struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat;
        }
        let mut m = vec![vec![0; c as usize]; r as usize];
        for row in 0..mat.len() {
            for col in 0..mat[0].len() {
                let total = row * mat[0].len() + col;
                let ri = total / c as usize;
                let ci = total % c as usize;
                m[ri][ci] = mat[row][col];
            }
        }
        m
    }
}

fn main() {
    println!("Hello, world!");
}
