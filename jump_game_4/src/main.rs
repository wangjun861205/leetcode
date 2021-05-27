use std::collections::HashMap;
use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn min_jumps_with_map(
        arr: Vec<i32>,
        index: i32,
        map: &mut HashMap<i32, i32>,
        set: &mut HashSet<i32>,
    ) -> i32 {
        set.insert(index);
        let val = arr[index as usize];
        let forward_index = index + 1;
        let backward_index = index - 1;
        let jump_indices: Vec<i32> = arr
            .iter()
            .enumerate()
            .filter(|(i, &v)| v == val && !set.contains(&(*i as i32)))
            .map(|(i, _)| i as i32)
            .collect();
        let len = arr.len();
        if forward_index == (len - 1) as i32
            || backward_index == (len - 1) as i32
            || jump_indices.contains(&(len as i32 - 1))
        {
            map.insert(index, 1);
            return 1;
        } else {
            let mut forward_step = -1;
            let mut backward_step = -1;
            if forward_index < len as i32 && !set.contains(&forward_index) {
                if map.contains_key(&forward_index) {
                    forward_step = map.get(&forward_index).unwrap().clone();
                } else {
                    forward_step =
                        Solution::min_jumps_with_map(arr.clone(), forward_index, map, set);
                }
            }
            if backward_index >= 0 && !set.contains(&backward_index) {
                if map.contains_key(&backward_index) {
                    backward_step = map.get(&backward_index).unwrap().clone();
                } else {
                    backward_step =
                        Solution::min_jumps_with_map(arr.clone(), backward_index, map, set);
                }
            }
            let mut jump_steps: Vec<i32> = jump_indices
                .into_iter()
                .map(|idx| {
                    if map.contains_key(&idx) {
                        map.get(&idx).unwrap().clone()
                    } else {
                        Solution::min_jumps_with_map(arr.clone(), idx, map, set)
                    }
                })
                .filter(|v| v > &0)
                .collect();
            if forward_step >= 0 {
                jump_steps.push(forward_step);
            }
            if backward_step >= 0 {
                jump_steps.push(backward_step);
            }
            let min_step = jump_steps.iter().min().or(Some(&-1)).unwrap().clone();
            if min_step < 0 {
                return -1;
            }
            map.insert(index, min_step + 1);
            println!("{:?}", map);
            min_step + 1
        }
    }
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut s: HashSet<i32> = HashSet::new();
        Solution::min_jumps_with_map(arr, 0, &mut m, &mut s)
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
    );
}
