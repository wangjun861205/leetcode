struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let min_pair = arr.windows(2).min_by_key(|w| w[1] - w[0]).unwrap();
        let min = min_pair[1] - min_pair[0];
        arr.windows(2)
            .filter(|w| w[1] - w[0] == min)
            .map(|w| w.to_vec())
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27])
    );
}
