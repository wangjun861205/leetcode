struct Solution;

impl Solution {
    fn collect(mat: &Vec<Vec<i32>>, mut i: usize, mut j: usize) -> Vec<i32> {
        let mut l: Vec<i32> = Vec::new();
        while i < mat.len() && j < mat[0].len() {
            l.push(mat[i][j]);
            i += 1;
            j += 1;
        }
        l
    }

    fn put(mat: &mut Vec<Vec<i32>>, mut i: usize, mut j: usize, mut list: Vec<i32>) {
        while !list.is_empty() {
            mat[i][j] = list.remove(0);
            i += 1;
            j += 1;
        }
    }

    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..mat.len() {
            let mut l = Solution::collect(&mat, i, 0);
            l.sort();
            Solution::put(&mut mat, i, 0, l);
        }
        for j in 0..mat[0].len() {
            let mut l = Solution::collect(&mat, 0, j);
            l.sort();
            Solution::put(&mut mat, 0, j, l);
        }
        mat
    }
}
fn main() {
    println!("Hello, world!");
}
