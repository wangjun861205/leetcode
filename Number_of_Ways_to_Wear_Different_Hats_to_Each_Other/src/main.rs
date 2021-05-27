struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(mut hats: Vec<HashSet<i32>>, picked: HashSet<i32>) -> i32 {
        if hats.len() == 0 {
            return 1;
        }
        let curr = hats.remove(0);
        let mut ans = 0;
        for d in curr.difference(&picked) {
            let mut p = picked.clone();
            p.insert(*d);
            ans += Solution::rc(hats.clone(), p);
        }
        ans
    }
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let h: Vec<HashSet<i32>> = hats.into_iter().map(|v| v.into_iter().collect()).collect();
        let p: HashSet<i32> = HashSet::new();
        Solution::rc(h, p)
    }
}
fn main() {
    println!(
        "{}",
        Solution::number_ways(vec![
            vec![2, 12, 18, 20, 24, 26, 30],
            vec![1, 2, 4, 5, 8, 9, 10, 11, 13, 15, 16, 17, 19, 20, 21, 22, 24, 26, 28, 29],
            vec![9, 10, 13, 14, 18, 27],
            vec![1, 2, 3, 7, 9, 15, 16, 18, 28],
            vec![2, 3, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 30],
            vec![2, 11, 14, 25],
            vec![2, 3, 5, 6, 7, 8, 9, 11, 13, 16, 17, 18, 19, 20, 21, 22, 24, 26, 27, 28, 29, 30]
        ])
    );
}
