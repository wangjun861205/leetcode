struct Solution;

impl Solution {
    fn move_stone(row: &mut Vec<char>) {
        let mut stone_count = 0;
        for i in 0..row.len() {
            match row[i] {
                '#' => {
                    stone_count += 1;
                    row[i] = '.'
                }
                '*' => {
                    for j in i - stone_count..i {
                        row[j] = '#';
                    }
                    stone_count = 0;
                }
                _ => {}
            }
        }
        if stone_count > 0 {
            for i in row.len() - stone_count..row.len() {
                row[i] = '#';
            }
        }
    }
    pub fn rotate_the_box(mut box_: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for r in 0..box_.len() {
            Solution::move_stone(&mut box_[r]);
        }
        let n = box_.len().max(box_[0].len());
        let mut matrix = vec![vec!['-'; n]; n];
        for i in 0..box_.len() {
            for j in 0..box_[0].len() {
                matrix[i][j] = box_[i][j];
            }
        }
        for i in 0..n {
            for j in 0..i {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..n {
            for j in 0..n / 2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[i][n - 1 - j];
                matrix[i][n - 1 - j] = temp;
            }
        }
        matrix
            .into_iter()
            .map(|row| row.into_iter().filter(|c| c != &'-').collect::<Vec<char>>())
            .filter(|row| !row.is_empty())
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']])
    );
}
