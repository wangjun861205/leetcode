struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let length = moves.len();
        let mut matrix = vec![vec![0; 3]; 3];
        for (i, m) in moves.into_iter().enumerate() {
            if i % 2 == 0 {
                matrix[m[0] as usize][m[1] as usize] = 1;
            } else {
                matrix[m[0] as usize][m[1] as usize] = -1;
            }
        }
        for i in 0..3 {
            let mut sum = 0;
            for j in 0..3 {
                sum += matrix[i][j];
            }
            if sum == 3 {
                return "A".to_owned();
            }
            if sum == -3 {
                return "B".to_owned();
            }
        }
        for j in 0..3 {
            let mut sum = 0;
            for i in 0..3 {
                sum += matrix[i][j];
            }
            if sum == 3 {
                return "A".to_owned();
            }
            if sum == -3 {
                return "B".to_owned();
            }
        }
        let d1 = matrix[0][0] + matrix[1][1] + matrix[2][2];
        if d1 == 3 {
            return "A".to_owned();
        }
        if d1 == -3 {
            return "B".to_owned();
        }
        let d2 = matrix[2][0] + matrix[1][1] + matrix[0][2];
        if d2 == 3 {
            return "A".to_owned();
        }
        if d2 == -3 {
            return "B".to_owned();
        }
        if length == 9 {
            return "Draw".to_owned();
        }
        "Pending".to_owned()
    }
}
fn main() {
    println!("Hello, world!");
}
