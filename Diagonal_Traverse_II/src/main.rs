struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let max_col = nums.iter().map(|l| l.len()).max().unwrap();
        let mut result: Vec<Vec<i32>> = vec![vec![]; nums.len() + max_col - 1];
        for (r, l) in nums.into_iter().enumerate() {
            for (c, v) in l.into_iter().enumerate() {
                result[r + c].insert(0, v)
            }
        }
        result.into_iter().flatten().collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_diagonal_order(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16]
        ])
    );
}
