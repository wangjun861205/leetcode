use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn can_reach_with_map(arr: Vec<i32>, start: i32, set: &mut HashSet<i32>) -> bool {
        if arr[start as usize] == 0 {
            return true;
        } else {
            set.insert(start);
            let right_index = start + arr[start as usize];
            if right_index < arr.len() as i32 && !set.contains(&right_index) {
                if Solution::can_reach_with_map(arr.clone(), right_index, set) {
                    return true;
                }
            }
            let left_index = start - arr[start as usize];
            if left_index >= 0 && !set.contains(&left_index) {
                if Solution::can_reach_with_map(arr.clone(), left_index, set) {
                    return true;
                }
            }
            false
        }
    }
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        Solution::can_reach_with_map(arr, start, &mut set)
    }
}
fn main() {
    println!("{}", Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
}
