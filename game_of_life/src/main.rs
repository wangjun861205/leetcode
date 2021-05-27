struct Solution;

impl Solution {
    pub fn trans_status(cell: &mut i32, live_count: i32) {
        match cell {
            0 => {
                if live_count == 3 {
                    *cell = 1;
                }
            }
            _ => {
                if live_count != 2 && live_count != 3 {
                    *cell = 0;
                }
            }
        }
    }

    pub fn live_count(board: Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        let mut count = 0;
        let row_len = board.len();
        let col_len = board[0].len();
        if row > 0 && col > 0 {
            count += board[row - 1][col - 1];
        }
        if row > 0 {
            count += board[row - 1][col];
        }
        if row > 0 && col < col_len - 1 {
            count += board[row - 1][col + 1];
        }
        if col > 0 {
            count += board[row][col - 1];
        }
        if col < col_len - 1 {
            count += board[row][col + 1];
        }
        if row < row_len - 1 && col > 0 {
            count += board[row + 1][col - 1];
        }
        if row < row_len - 1 {
            count += board[row + 1][col];
        }
        if row < row_len - 1 && col < col_len - 1 {
            count += board[row + 1][col + 1];
        }
        count
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let board_clone = board.clone();
        board_clone.iter().enumerate().for_each(|(i, row)| {
            row.into_iter().enumerate().for_each(|(j, cell)| {
                let live_count = Solution::live_count(board_clone.clone(), i, j);
                Solution::trans_status(&mut board[i][j], live_count);
            });
        })
    }
}
fn main() {
    let mut m = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(&mut m);
    println!("{:?}", m);
}
