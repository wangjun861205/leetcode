struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans: usize = 0;
        for i in 0..arr.len() {
            let mut res = arr[i];
            for j in i + 1..arr.len() {
                if res ^ arr[j] == 0 {
                    ans += j - i;
                }
                res ^= arr[j];
            }
        }
        ans as i32
    }
}
fn main() {
    println!("Hello, world!");
}
