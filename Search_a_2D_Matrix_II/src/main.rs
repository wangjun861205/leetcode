struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0 as i32;
        let mut col = matrix[0].len() as i32 - 1;
        while row < matrix.len() as i32 && col >= 0 {
            let val = matrix[row as usize][col as usize];
            if val == target {
                return true;
            }
            if val > target {
                col -= 1;
            }
            if val < target {
                row += 1;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
