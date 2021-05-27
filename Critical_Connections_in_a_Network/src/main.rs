struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(conns: &mut Vec<Vec<i32>>, target: i32, stack: &mut Vec<(Vec<i32>, bool)>, set: &mut HashSet<i32>) {
        let mut next: Vec<Vec<i32>> = Vec::new();
        while let Some(idx) = conns.iter().position(|v| v[0] == target || v[1] == target) {
            next.push(conns.remove(idx));
        }
        for n in next {
            if n[0] == target {
                stack.push((vec![n[0], n[1]], false));
                if set.contains(&n[1]) {
                    for (e, v) in stack.iter_mut().rev() {
                        *v = true;
                        if e[0] == n[1] {
                            break;
                        }
                    }
                } else {
                    set.insert(n[1]);
                    Solution::rc(conns, n[1], stack, set);
                }
            } else {
                stack.push((vec![n[1], n[0]], false));
                if set.contains(&n[0]) {
                    for (e, v) in stack.iter_mut().rev() {
                        *v = true;
                        if e[0] == n[0] {
                            break;
                        }
                    }
                } else {
                    set.insert(n[0]);
                    Solution::rc(conns, n[0], stack, set);
                }
            }
        }
    }
    pub fn critical_connections(n: i32, mut connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut stack: Vec<(Vec<i32>, bool)> = Vec::new();
        let mut set: HashSet<i32> = HashSet::new();
        let start = connections.pop().unwrap();
        let target = start[1];
        set.insert(start[0]);
        set.insert(start[1]);
        stack.push((start, false));
        Solution::rc(&mut connections, target, &mut stack, &mut set);
        println!("{:?}", stack);
        stack.into_iter().filter_map(|(e, v)| if !v { Some(e) } else { None }).collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3], vec![3, 4], vec![4, 5], vec![5, 3]])
    );
}

// 4->0->1->4!
//    |
//    |->2->4!
//    |
//    |->3
