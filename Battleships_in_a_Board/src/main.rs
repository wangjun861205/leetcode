struct Solution;

impl Solution {
    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if board[r][c] == 'X' {
                    board[r][c] = '-';
                    let left = if c > 0 { board[r][c - 1] == '-' } else { false };
                    let right = if c < board[0].len() - 1 {
                        board[r][c + 1] == '-'
                    } else {
                        false
                    };
                    let top = if r > 0 { board[r - 1][c] == '-' } else { false };
                    let bottom = if r < board.len() - 1 {
                        board[r + 1][c] == '-'
                    } else {
                        false
                    };
                    if !(left || right || top || bottom) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_battleships(vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X']
        ])
    );
}
