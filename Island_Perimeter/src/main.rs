struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut land = 0;
        let mut bound = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    land += 1;
                    if r != 0 && grid[r - 1][c] == 1 {
                        bound += 1;
                    }
                    if c != 0 && grid[r][c - 1] == 1 {
                        bound += 1;
                    }
                }
            }
        }
        land * 4 - bound * 2
    }
}

fn main() {
    println!("Hello, world!");
}
