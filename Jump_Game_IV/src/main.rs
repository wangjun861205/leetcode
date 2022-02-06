struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut conns = arr.iter().enumerate().fold(HashMap::new(), |mut m, (i, v)| {
            m.entry(*v).or_insert(Vec::new()).push(i);
            m
        });
        let mut curr = vec![0_usize];
        let mut next = vec![];
        let mut visited: HashSet<usize> = vec![0].into_iter().collect();
        let mut step = 0;
        while !curr.is_empty() {
            for node in curr {
                if node == arr.len() - 1 {
                    return step;
                }
                if let Some(siblings) = conns.get(&arr[node]) {
                    for s in siblings {
                        if !visited.contains(s) {
                            next.push(*s);
                            visited.insert(*s);
                        }
                    }
                }
                conns.remove(&arr[node]);
                for i in node as i32 - 1..=node as i32 + 1 {
                    if i >= 0 && i < arr.len() as i32 && !visited.contains(&(i as usize)) {
                        visited.insert(i as usize);
                        next.push(i as usize);
                    }
                }
            }
            curr = next;
            next = vec![];
            step += 1;
        }
        unreachable!()
    }
}

fn main() {
    println!("{}", Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]));
}
