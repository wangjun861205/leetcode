struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut parents: Vec<usize> = (0..is_connected.len()).collect();
        let mut updated = false;
        loop {
            for i in 0..parents.len() {
                for (j, &c) in is_connected[i].iter().enumerate() {
                    if c == 1 {
                        if parents[j] > parents[i] {
                            parents[j] = parents[i];
                            let parent_j = parents[j];
                            parents[parent_j] = parents[i];
                            updated = true;
                        }
                    }
                }
            }
            if !updated {
                break;
            }
            updated = false;
        }
        parents.into_iter().collect::<HashSet<usize>>().len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
    );
}
