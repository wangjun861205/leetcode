struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let m: HashMap<char, (usize, usize)> = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, (i / 5, i % 5)))
            .collect();
        let (mut row, mut col) = (0_usize, 0_usize);
        let mut ans = String::new();
        let mut z_out = false;
        for chr in target.chars() {
            let (r, c) = m.get(&chr).unwrap();
            if z_out {
                if *r < row {
                    for _ in 0..row - *r {
                        ans.push('U');
                    }
                } else if *r > row {
                    for _ in 0..*r - row {
                        ans.push('D');
                    }
                }
                if *c < col {
                    for _ in 0..col - *c {
                        ans.push('L');
                    }
                } else if *c > col {
                    for _ in 0..*c - col {
                        ans.push('R');
                    }
                }
                z_out = false;
            } else {
                if *c < col {
                    for _ in 0..col - *c {
                        ans.push('L');
                    }
                } else if *c > col {
                    for _ in 0..*c - col {
                        ans.push('R');
                    }
                }
                if *r < row {
                    for _ in 0..row - *r {
                        ans.push('U');
                    }
                } else if *r > row {
                    for _ in 0..*r - row {
                        ans.push('D');
                    }
                }
            }
            ans.push('!');
            row = *r;
            col = *c;
            if chr == 'z' {
                z_out = true;
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::alphabet_board_path("leet".to_string()));
}
