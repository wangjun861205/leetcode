struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut set: HashSet<String> = HashSet::new();
        set.insert("".into());
        for c in tiles.chars() {
            let new_set = set.clone();
            for w in new_set {
                for i in 0..=w.len() {
                    let mut wc = w.clone();
                    wc.insert(i, c);
                    set.insert(wc);
                }
                let mut wc = w.clone();
                wc.push(c);
                set.insert(wc);
            }
        }
        set.len() as i32 - 1
    }
}

fn main() {
    println!("{}", Solution::num_tile_possibilities("AAABBC".into()));
}
