struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let M = 1000000007_i128;
        arr.sort();
        let pos: HashMap<i32, usize> = arr.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let mut ans = vec![1_i128; arr.len()];
        for i in 0..arr.len() {
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    let v = arr[i] / arr[j];
                    if let Some(p) = pos.get(&v) {
                        ans[i] += ans[j] * ans[*p] % M;
                    }
                }
            }
        }
        let mut res: i128 = 0;
        for v in ans {
            res += v % M;
            res %= M;
        }
        res as i32
    }
}
fn main() {
    println!("{}", Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
}
