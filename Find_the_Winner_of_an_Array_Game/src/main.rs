struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut prev = arr[0];
        let mut counts = HashMap::new();
        for v in arr.into_iter().skip(1) {
            if v < prev {
                let ent = counts.entry(prev).or_insert(0);
                *ent += 1;
                if *ent == k {
                    break;
                }
            } else {
                prev = v;
                let ent = counts.entry(v).or_insert(0);
                *ent += 1;
                if *ent == k {
                    break;
                }
            }
        }
        prev
    }
}

fn main() {
    println!("{}", Solution::get_winner(vec![1, 25, 35, 68, 42, 70], 2));
}
