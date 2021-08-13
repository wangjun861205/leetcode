struct Solution;

use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        arr.sort_by_key(|v| v.abs());
        let mut map: HashMap<i32, i32> = arr.iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
        for v in arr {
            if map.get(&v).unwrap() == &0 {
                continue;
            }
            *map.get_mut(&v).unwrap() -= 1;

            if !map.contains_key(&(v * 2)) || map.get(&(v * 2)).unwrap() == &0 {
                return false;
            }
            *map.get_mut(&(v * 2)).unwrap() -= 1;
        }
        true
    }
}

fn main() {
    println!("{}", Solution::can_reorder_doubled(vec![0, 0]));
}
