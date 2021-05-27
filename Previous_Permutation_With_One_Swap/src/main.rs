struct Solution;

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let index = arr
            .windows(2)
            .enumerate()
            .rev()
            .find(|(_, w)| w[0] > w[1])
            .map(|(i, _)| i);
        if let Some(i) = index {
            for j in (i + 1..arr.len()).rev() {
                if arr[j] < arr[i] && arr[j] != arr[j - 1] {
                    arr.swap(i, j);
                    break;
                }
            }
        }
        arr
    }
}
fn main() {
    println!("{:?}", Solution::prev_perm_opt1(vec![3, 1, 1, 3]));
}
