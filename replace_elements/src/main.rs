struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = arr[arr.len() - 1];
        let len = arr.len();
        for (i, v) in arr.clone().into_iter().rev().enumerate().skip(1) {
            arr[len - 1 - i] = max;
            if v > max {
                max = v;
            }
        }
        arr[len - 1] = -1;
        arr
    }
}
fn main() {
    println!("{:?}", Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]));
}
