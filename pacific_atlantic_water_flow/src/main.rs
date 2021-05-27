struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn walk(
        x: usize,
        y: usize,
        matrix: &Vec<Vec<i32>>,
        m: &mut Vec<Vec<bool>>,
        passed: &mut Vec<Vec<bool>>,
    ) {
        m[y][x] = true;
        passed[y][x] = true;
        if x != 0 && !passed[y][x - 1] && !m[y][x - 1] && matrix[y][x - 1] >= matrix[y][x] {
            Solution::walk(x - 1, y, matrix, m, passed);
        }
        if y != 0 && !passed[y - 1][x] && !m[y - 1][x] && matrix[y - 1][x] >= matrix[y][x] {
            Solution::walk(x, y - 1, matrix, m, passed);
        }
        if x != matrix[0].len() - 1
            && !passed[y][x + 1]
            && !m[y][x + 1]
            && matrix[y][x + 1] >= matrix[y][x]
        {
            Solution::walk(x + 1, y, matrix, m, passed);
        }
        if y != matrix.len() - 1
            && !passed[y + 1][x]
            && !m[y + 1][x]
            && matrix[y + 1][x] >= matrix[y][x]
        {
            Solution::walk(x, y + 1, matrix, m, passed);
        }
    }
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.len() == 0 {
            return matrix;
        }
        let mut pm = vec![vec![false; matrix[0].len()]; matrix.len()];
        let mut am = vec![vec![false; matrix[0].len()]; matrix.len()];
        for y in 0..matrix.len() {
            let mut pp = pm.clone();
            Solution::walk(0, y, &matrix, &mut pm, &mut pp);
        }
        for x in 0..matrix[0].len() {
            let mut pp = pm.clone();
            Solution::walk(x, 0, &matrix, &mut pm, &mut pp);
        }
        for y in 0..matrix.len() {
            let mut ap = am.clone();
            Solution::walk(matrix[0].len() - 1, y, &matrix, &mut am, &mut ap);
        }
        for x in 0..matrix[0].len() {
            let mut ap = am.clone();
            Solution::walk(x, matrix.len() - 1, &matrix, &mut am, &mut ap);
        }
        println!("{:?}", pm);
        println!("{:?}", am);
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                if pm[y][x] && am[y][x] {
                    ans.push(vec![y as i32, x as i32]);
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ])
    )
}
