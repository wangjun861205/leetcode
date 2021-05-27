struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn rc(
        times: &HashMap<i32, Vec<(i32, i32)>>,
        node: i32,
        time: i32,
        mut visited: HashSet<i32>,
        cache: &mut HashMap<i32, i32>,
    ) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        if let Some(c) = cache.get_mut(&node) {
            if *c > time {
                *c = time;
            } else {
                return;
            }
        } else {
            cache.insert(node, time);
        }
        if let Some(nexts) = times.get(&node) {
            for next in nexts {
                Solution::rc(times, next.0, time + next.1, visited.clone(), cache);
            }
        }
    }
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut ts: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for t in times {
            ts.entry(t[0]).or_insert(Vec::new()).push((t[1], t[2]));
        }
        let mut cache: HashMap<i32, i32> = HashMap::new();
        Solution::rc(&ts, k, 0, HashSet::new(), &mut cache);
        if cache.len() < n as usize {
            return -1;
        }
        *cache.values().max().unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 1, 3]], 2, 2)
    );
}
