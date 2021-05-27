struct Solution;

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut l: _ = Vec::new();
        let mut dir = "up";
        let mut i = 0_i32;
        let mut j = 0_i32;
        let rows = matrix.len() as i32;
        if rows == 0 {
            return l;
        }
        let cols = matrix[0].len() as i32;
        loop {
            // 向上出格
            if i < 0 {
                // 右方未出格
                if j < cols {
                    i += 1;
                } else {
                    // 右方也出格
                    i += 2;
                    j -= 1;
                }
                dir = "down";
            }
            // 向下出格
            if i == rows {
                i -= 1;
                j += 2;
                dir = "up";
            }
            // 向左出格
            if j < 0 {
                j += 1;
                dir = "up";
            }
            // 向右出格
            if j == cols {
                i += 2;
                j -= 1;
                dir = "down";
            }
            l.push(matrix[i as usize][j as usize]);
            if i == rows - 1 && j == cols - 1 {
                break;
            }
            if dir == "up" {
                i -= 1;
                j += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        l
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_diagonal_order(vec![vec![2, 5], vec![8, 4], vec![0, -1]])
    );
}
