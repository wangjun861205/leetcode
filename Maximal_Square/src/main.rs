struct Solution;

impl Solution {
    fn max(matrix: &Vec<Vec<char>>, left: usize, top: usize, right: usize, bottom: usize) -> i32 {
        let area = (bottom - top + 1) as i32 * (right - left + 1) as i32;
        if right == matrix[0].len() - 1 || bottom == matrix.len() - 1 {
            return area;
        }

        for i in top..=bottom {
            if matrix[i][right + 1] == '0' {
                return area;
            }
        }
        for j in left..=right + 1 {
            if matrix[bottom + 1][j] == '0' {
                return area;
            }
        }
        Solution::max(matrix, left, top, right + 1, bottom + 1)
    }

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    ans = ans.max(Solution::max(&matrix, j, i, j, i));
                }
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ])
    );
}
