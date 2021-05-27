struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_bottom(
        mat: Vec<Vec<Rc<RefCell<i32>>>>,
        row: usize,
        col: usize,
    ) -> Vec<Rc<RefCell<i32>>> {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut l: Vec<Rc<RefCell<i32>>> = Vec::new();
        l.push(mat[row][col].clone());
        if row + 1 < rows && col + 1 < cols {
            l.append(&mut Solution::right_bottom(mat.clone(), row + 1, col + 1));
        }
        l
    }
    pub fn sort(mat: Vec<Vec<Rc<RefCell<i32>>>>) {
        let mut mat_clone: Vec<Vec<i32>> = mat
            .iter()
            .map(|row| row.iter().map(|col| *col.borrow()).collect())
            .collect();
        for row in mat_clone.iter_mut() {
            row.sort();
        }
        for (i, row) in mat_clone.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let cl = mat[i][j].clone();
                *cl.borrow_mut() = *col;
            }
        }
    }
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mat_rc: Vec<Vec<Rc<RefCell<i32>>>> = mat
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|col| Rc::new(RefCell::new(col)))
                    .collect()
            })
            .collect();
        let mut diagonal: Vec<Vec<Rc<RefCell<i32>>>> = Vec::new();
        for (i, row) in mat_rc.iter().enumerate() {
            match i {
                0 => {
                    for (j, _) in row.iter().enumerate() {
                        diagonal.push(Solution::right_bottom(mat_rc.clone(), i, j));
                    }
                }
                _ => {
                    diagonal.push(Solution::right_bottom(mat_rc.clone(), i, 0));
                }
            }
        }
        Solution::sort(diagonal);
        mat_rc
            .into_iter()
            .map(|row| row.into_iter().map(|v| *v.borrow()).collect())
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::diagonal_sort(vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]])
    );
}
