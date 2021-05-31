struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let map: HashMap<String, HashSet<usize>> =
            favorite_companies
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut s, (i, v)| {
                    v.iter().for_each(|v| {
                        s.entry(v.clone()).or_insert(HashSet::new()).insert(i);
                    });
                    s
                });
        let ans: Vec<i32> =
            favorite_companies
                .into_iter()
                .enumerate()
                .fold(Vec::new(), |mut l, (i, v)| {
                    let mut set: HashSet<usize> = HashSet::new();
                    for c in v {
                        let s = map.get(&c).unwrap();
                        if set.len() == 0 {
                            set = s.clone();
                        } else {
                            set = set.intersection(s).map(|idx| *idx).collect();
                        }
                    }
                    if set.len() == 1 {
                        l.push(i as i32);
                    }
                    l
                });
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
