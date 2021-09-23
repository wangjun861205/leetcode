struct Solution;

use std::collections::HashSet;

impl Solution {
    fn check(ori: i32, cur: i32, dls: &Vec<HashSet<i32>>, is_dislike: bool) -> bool {
        if is_dislike {
            for dl in &dls[cur as usize] {
                for lk in &dls[*dl as usize] {
                    if !Solution::check(ori, *lk, dls, false) {
                        return false;
                    }
                }
            }
        } else {
            if !dls[ori as usize].contains(&cur) {
                return false;
            }
            for dl in &dls[cur as usize] {
                if !Solution::check(ori, *dl, dls, true) {
                    return false;
                }
            }
        }
        true
    }
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut dls: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize + 1];
        for dl in &dislikes {
            dls[dl[0] as usize].insert(dl[1]);
            dls[dl[1] as usize].insert(dl[0]);
        }
        for i in 1..=n {
            if !Solution::check(i, i, &dls, true) {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]])
    );
}
