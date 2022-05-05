struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut edges = vec![BinaryHeap::new(); points.len()];
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let pi = points[i].clone();
                let pj = points[j].clone();
                let dist = (pi[0] - pj[0]).abs() + (pi[1] - pj[1]).abs();
                edges[i].push(Reverse((dist, j)));
                edges[j].push(Reverse((dist, i)));
            }
        }
        let mut visited = vec![false; points.len()];
        let mut count = 0;
        let mut ans = 0;
        let mut stack = edges[0].clone();
        visited[0] = true;
        'outer: while count < points.len() - 1 {
            while let Some(Reverse((dist, i))) = stack.pop() {
                if visited[i] {
                    continue 'outer;
                }
                visited[i] = true;
                count += 1;
                ans += dist;
                stack.append(&mut edges[i].clone());
                continue 'outer;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_cost_connect_points(vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0]
        ])
    );
}
