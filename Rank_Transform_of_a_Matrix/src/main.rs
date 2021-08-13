struct Solution;

use std::cell::RefCell;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    val: i32,
    row: usize,
    col: usize,
    rank: i32,
    parent_row: usize,
    parent_col: usize,
}

impl Cell {
    fn new(val: i32, row: usize, col: usize) -> Self {
        Self {
            val,
            row,
            col,
            rank: 0,
            parent_row: row,
            parent_col: col,
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Solution {
    fn find(parents: &Vec<Vec<Rc<RefCell<Cell>>>>, cell: Rc<RefCell<Cell>>) -> Rc<RefCell<Cell>> {
        if (cell.borrow().row, cell.borrow().col) == (cell.borrow().parent_row, cell.borrow().parent_col) {
            return cell;
        }
        let p = parents[cell.borrow().parent_row][cell.borrow().parent_col].clone();
        Solution::find(parents, p)
    }

    fn union(parents: &Vec<Vec<Rc<RefCell<Cell>>>>, cell1: Rc<RefCell<Cell>>, cell2: Rc<RefCell<Cell>>) {
        let p1 = Solution::find(parents, cell1);
        let p2 = Solution::find(parents, cell2);
        p2.borrow_mut().parent_row = p1.borrow().row;
        p2.borrow_mut().parent_col = p1.borrow().col;
    }

    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<Rc<RefCell<Cell>>>> = matrix
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, r)| r.into_iter().enumerate().map(|(j, c)| Rc::new(RefCell::new(Cell::new(c, i, j)))).collect())
            .collect();
        let mut row_maxes: Vec<Option<Rc<RefCell<Cell>>>> = vec![None; matrix.len()];
        let mut col_maxes: Vec<Option<Rc<RefCell<Cell>>>> = vec![None; matrix[0].len()];
        let mut heap: BinaryHeap<Reverse<Rc<RefCell<Cell>>>> = ans.iter().flat_map(|row| row.iter().map(|c| Reverse(c.clone()))).collect();
        while let Some(Reverse(cell)) = heap.pop() {
            let row = cell.borrow().row;
            let col = cell.borrow().col;
            let val = cell.borrow().val;
            if let Some(row_max) = &row_maxes[row] {
                if let Some(col_max) = &col_maxes[col] {
                    if val == row_max.borrow().val {
                        if val == col_max.borrow().val {
                            let row_parent = Solution::find(&ans, row_max.clone());
                            let col_parent = Solution::find(&ans, col_max.clone());
                            let rank = row_parent.borrow().rank.max(col_parent.borrow().rank);
                            Solution::union(ans, row_parent.clone(), col_parent.clone());
                            Solution::union(ans, row_parent.clone(), cell.clone());
                            row_parent.borrow_mut().rank = rank;
                            cell.borrow_mut().rank = rank;
                        } else {
                            Solution::union(ans, row_parent.clone())
                        }
                    }
                }
            }
        }
        vec![vec![]]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::matrix_rank_transform(vec![
            vec![-23, 20, -49, -30, -39, -28, -5, -14],
            vec![-19, 4, -33, 2, -47, 28, 43, -6],
            vec![-47, 36, -49, 6, 17, -8, -21, -30],
            vec![-27, 44, 27, 10, 21, -8, 3, 14],
            vec![-19, 12, -25, 34, -27, -48, -37, 14],
            vec![-47, 40, 23, 46, -39, 48, -41, 18],
            vec![-27, -4, 7, -10, 9, 36, 43, 2],
            vec![37, 44, 43, -38, 29, -44, 19, 38]
        ])
    );
}

// expected:
// [
// [7,13,1,5,4,6,9,8]
// ,[8,11,2,10,1,12,14,9]
// ,[2,14,1,11,13,7,5,3]
// ,[3,19,16,12,14,7,10,13]
// ,[8,12,6,14,5,1,4,13]
// ,[2,16,15,17,4,18,3,14]
// ,[3,7,11,6,12,13,14,10]
// ,[16,19,18,3,15,2,11,17]
// ]

// output:
// [
// [6,12,1,4,3,5,8,7]
// ,[7,10,2,9,1,11,13,8]
// ,[1,13,1,10,12,6,4,2]
// ,[2,18,15,11,13,6,9,12]
// ,[7,11,5,13,4,1,3,12]
// ,[1,15,14,16,3,17,2,13]
// ,[2,6,10,5,11,12,13,9]
// ,[15,18,17,3,14,2,10,16]
// ]
