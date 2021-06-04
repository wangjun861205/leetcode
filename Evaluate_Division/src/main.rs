struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let equations: HashMap<String, Vec<(String, f64)>> = equations
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut m, (i, v)| {
                m.entry(v[0].clone())
                    .or_insert(Vec::new())
                    .push((v[1].clone(), values[i]));
                m.entry(v[1].clone())
                    .or_insert(Vec::new())
                    .push((v[0].clone(), 1.0 / values[i]));
                m
            });
        queries
            .into_iter()
            .map(|q| {
                if let Some(mut queue) = equations.get(&q[0]).cloned() {
                    let mut visited: HashSet<String> = HashSet::new();
                    while !queue.is_empty() {
                        for _ in 0..queue.len() {
                            let node = queue.remove(0);
                            if node.0 == q[1] {
                                return node.1;
                            }
                            if visited.contains(&node.0) {
                                continue;
                            }
                            visited.insert(node.0.clone());
                            let mut nexts = equations
                                .get(&node.0)
                                .unwrap()
                                .clone()
                                .into_iter()
                                .map(|mut v| {
                                    v.1 = v.1 * node.1;
                                    v
                                })
                                .collect();
                            queue.append(&mut nexts);
                        }
                    }
                    -1.0
                } else {
                    -1.0
                }
            })
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
