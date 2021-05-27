struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut matrix: Vec<Vec<char>> = vec![vec!['-'; s.len()]; num_rows as usize];
        let mut row = 0_usize;
        let mut col = 0_usize;
        let mut direct = "down".to_owned();
        let mut s: Vec<char> = s.chars().collect();
        while s.len() > 0 {
            let c = s.remove(0);
            matrix[row][col] = c;
            if row == 0 {
                row += 1;
                direct = "down".to_owned();
            } else if row == num_rows as usize - 1 {
                row -= 1;
                direct = "up".to_owned();
            } else {
                if &direct == "up" {
                    row -= 1;
                } else {
                    row += 1;
                }
            }
            col += 1;
        }
        matrix
            .into_iter()
            .map(|r| r.into_iter().filter(|c| *c != '-').collect::<String>())
            .collect::<Vec<String>>()
            .join("")
    }
}
fn main() {
    // println!("Hello, world!");
    println!("{}", Solution::convert("PAYPALISHIRING".to_owned(), 2));
}
