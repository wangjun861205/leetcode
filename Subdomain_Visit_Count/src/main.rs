struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        for domain in cpdomains
            .into_iter()
            .map(|v| v.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>())
        {
            let rep: i32 = domain[0].parse().unwrap();
            let mut comps = domain[1]
                .split(".")
                .map(str::to_owned)
                .collect::<Vec<String>>();
            let mut s = comps.pop().unwrap();
            *counts.entry(s.clone()).or_insert(0) += rep;
            while !comps.is_empty() {
                s.insert(0, '.');
                s.insert_str(0, &comps.pop().unwrap());
                *counts.entry(s.clone()).or_insert(0) += rep;
            }
        }
        counts
            .into_iter()
            .map(|(k, v)| format!("{} {}", v, k))
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
