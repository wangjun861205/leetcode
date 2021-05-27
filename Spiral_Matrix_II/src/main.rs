struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut row = 0_usize;
        let mut col = 0_usize;
        let mut dir = Direction::Right;
        let mut num = 1;
        while row < n as usize && col < n as usize {
            if matrix[row][col] == 0 {
                matrix[row][col] = num;
                num += 1;
            } else {
                break;
            }
            match dir {
                Direction::Right => {
                    if col == n as usize - 1 || matrix[row][col + 1] > 0 {
                        dir = Direction::Down;
                        row += 1;
                    } else {
                        col += 1;
                    }
                }
                Direction::Down => {
                    if row == n as usize - 1 || matrix[row + 1][col] > 0 {
                        dir = Direction::Left;
                        col -= 1;
                    } else {
                        row += 1;
                    }
                }
                Direction::Left => {
                    if col == 0 || matrix[row][col - 1] > 0 {
                        dir = Direction::Up;
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                }
                Direction::Up => {
                    if row == 0 || matrix[row - 1][col] > 0 {
                        dir = Direction::Right;
                        col += 1;
                    } else {
                        row -= 1;
                    }
                }
            }
        }
        matrix
    }
}
fn main() {
    println!("Hello, world!");
}
