struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let chars: Vec<Vec<char>> = board.into_iter().map(|r| r.chars().collect()).collect();
        let mut x_count = 0;
        let mut o_count = 0;
        chars.iter().for_each(|r| {
            r.iter().for_each(|c| {
                if c == &'X' {
                    x_count += 1;
                } else if c == &'O' {
                    o_count += 1;
                }
            })
        });
        if !(x_count - o_count == 1 || x_count == o_count) {
            return false;
        }
        for r in &chars {
            if r.iter().all(|v| v == &'X') {
                if x_count == o_count {
                    return false;
                }
            }
            if r.iter().all(|v| v == &'O') {
                if x_count - o_count == 1 {
                    return false;
                }
            }
        }
        for (x, (y, z)) in chars[0].iter().zip(chars[1].iter().zip(chars[2].iter())) {
            if x == &'X' && y == &'X' && z == &'X' {
                if x_count == o_count {
                    return false;
                }
            }
            if x == &'O' && y == &'O' && z == &'O' {
                if x_count - o_count == 1 {
                    return false;
                }
            }
        }
        if &chars[0][0] == &'X' && &chars[1][1] == &'X' && &chars[2][2] == &'X' {
            if x_count == o_count {
                return false;
            }
        }
        if &chars[0][0] == &'O' && &chars[1][1] == &'O' && &chars[2][2] == &'O' {
            if x_count - o_count == 1 {
                return false;
            }
        }

        if &chars[0][2] == &'X' && &chars[1][1] == &'X' && &chars[2][0] == &'X' {
            if x_count == o_count {
                return false;
            }
        }
        if &chars[0][2] == &'O' && &chars[1][1] == &'O' && &chars[2][0] == &'O' {
            if x_count - o_count == 1 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
