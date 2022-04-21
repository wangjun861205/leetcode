struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let row_length = grid[0].len();
        let total = grid.len() * grid[0].len();
        let k = k as usize % total;
        let l: Vec<i32> = grid.into_iter().flatten().collect();
        let mut new_list = l[l.len() - k..].to_vec();
        new_list.extend(&l[..l.len() - k]);
        new_list.chunks(row_length).map(|l| l.to_vec()).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
