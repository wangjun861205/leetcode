struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_bottom_right(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 + 1 <= grid.len() as i32 - 1 && coordinate.1 + 1 <= grid[0].len() as i32 - 1
        {
            return grid[coordinate.0 as usize + 1][coordinate.1 as usize + 1];
        }
        -1
    }

    pub fn check_right(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.1 + 1 <= grid.len() as i32 - 1 {
            return grid[coordinate.0 as usize][coordinate.1 as usize + 1];
        }
        -1
    }

    pub fn check_bottom(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 + 1 <= grid.len() as i32 - 1 {
            return grid[coordinate.0 as usize + 1][coordinate.1 as usize];
        }
        -1
    }

    pub fn check_bottom_left(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 + 1 <= grid.len() as i32 - 1 && coordinate.1 - 1 >= 0 {
            return grid[coordinate.0 as usize + 1][coordinate.1 as usize - 1];
        }
        -1
    }

    pub fn check_left(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.1 - 1 >= 0 {
            return grid[coordinate.0 as usize][coordinate.1 as usize - 1];
        }
        -1
    }

    pub fn check_top_left(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 - 1 >= 0 && coordinate.1 - 1 >= 0 {
            return grid[coordinate.0 as usize - 1][coordinate.1 as usize - 1];
        }
        -1
    }

    pub fn check_top(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 - 1 >= 0 {
            return grid[coordinate.0 as usize - 1][coordinate.1 as usize];
        }
        -1
    }

    pub fn check_top_right(coordinate: (i32, i32), grid: Vec<Vec<i32>>) -> i32 {
        if coordinate.0 - 1 >= 0 && coordinate.1 + 1 <= grid[0].len() as i32 - 1 {
            return grid[coordinate.0 as usize - 1][coordinate.1 as usize + 1];
        }
        -1
    }

    pub fn shortest_path_binary_matrix_with_cache(
        coordinate: (i32, i32),
        grid: Vec<Vec<i32>>,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if coordinate.0 == grid.len() - 1 && cordinate.1 == grid[0].len() - 1 {
            return 0;
        }
    }

    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {}
}

fn main() {
    println!("Hello, world!");
}
