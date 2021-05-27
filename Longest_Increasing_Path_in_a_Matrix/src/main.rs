struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(
        matrix: Vec<Vec<i32>>,
        start_point: (usize, usize),
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let x_boundary = matrix[0].len();
        let y_boundary = matrix.len();
        let x = start_point.1;
        let y = start_point.0;
        let mut l: Vec<i32> = Vec::new();
        if x + 1 < x_boundary && matrix[y][x + 1] > matrix[y][x] {
            if let Some(v) = cache.get(&(y, x + 1)) {
                l.push(*v);
            } else {
                l.push(Solution::rc(matrix.clone(), (y, x + 1), cache));
            }
        }
        if x != 0 && matrix[y][x - 1] > matrix[y][x] {
            if let Some(v) = cache.get(&(y, x - 1)) {
                l.push(*v)
            } else {
                l.push(Solution::rc(matrix.clone(), (y, x - 1), cache));
            }
        }
        if y + 1 < y_boundary && matrix[y + 1][x] > matrix[y][x] {
            if let Some(v) = cache.get(&(y + 1, x)) {
                l.push(*v);
            } else {
                l.push(Solution::rc(matrix.clone(), (y + 1, x), cache));
            }
        }
        if y != 0 && matrix[y - 1][x] > matrix[y][x] {
            if let Some(v) = cache.get(&(y - 1, x)) {
                l.push(*v);
            } else {
                l.push(Solution::rc(matrix.clone(), (y - 1, x), cache));
            }
        }
        if l.len() == 0 {
            cache.insert((y, x), 1);
            return 1;
        }
        let ans = l.into_iter().max().unwrap() + 1;
        cache.insert((y, x), ans);
        ans
    }
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut l: Vec<i32> = Vec::new();
        let y_boundary = matrix.len();
        let x_boundary = matrix[0].len();
        let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
        for y in 0..y_boundary {
            for x in 0..x_boundary {
                l.push(Solution::rc(matrix.clone(), (y, x), &mut cache));
            }
        }
        l.into_iter().max().unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]])
    );
}
