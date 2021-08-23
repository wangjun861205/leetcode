struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let (target_row, target_col) = (target[0], target[1]);
        let mine_distance = target_row.abs() + target_col.abs();
        for g in ghosts {
            let (ghost_row, ghost_col) = (g[0], g[1]);
            let ghost_distance = (target_row - ghost_row).abs() + (target_col - ghost_col).abs();
            if ghost_distance <= mine_distance {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
