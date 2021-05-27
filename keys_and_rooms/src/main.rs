struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rc(index: usize, rooms: Vec<Vec<i32>>, cache: &mut HashMap<usize, Vec<i32>>) {
        if cache.contains_key(&index) {
            return;
        }
        let keys = rooms[index].clone();
        cache.insert(index, keys.clone());
        for key in keys.clone() {
            Solution::rc(key as usize, rooms.clone(), cache);
        }
    }
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut cache: HashMap<usize, Vec<i32>> = HashMap::new();
        Solution::rc(0, rooms.clone(), &mut cache);
        cache.len() == rooms.len()
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]])
    );
}
