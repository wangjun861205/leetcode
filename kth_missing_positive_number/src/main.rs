use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        match arr.len() {
            1 => {
                if k < arr[0] {
                    k
                } else {
                    k + 1
                }
            }
            _ => {
                let end = arr[arr.len() - 1];
                let set: HashSet<i32> = arr.into_iter().collect();
                let lost: Vec<i32> = (1..end).into_iter().filter(|v| !set.contains(v)).collect();
                if lost.len() >= k as usize {
                    lost[k as usize - 1]
                } else {
                    end + (k - lost.len() as i32)
                }
            }
        }
    }
}
fn main() {
    println!("{}", Solution::find_kth_positive(vec![1, 13, 18], 17));
}
