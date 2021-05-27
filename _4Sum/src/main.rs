struct Solution;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Tuple(i32, i32, i32, i32);

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sum: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                sum.entry(nums[i] + nums[j])
                    .or_insert(Vec::new())
                    .push((i, j));
            }
        }
        let mut set: HashSet<Tuple> = HashSet::new();
        for (k, v) in &sum {
            if let Some(vv) = sum.get(&(target - *k)) {
                for i in 0..v.len() {
                    for j in 0..vv.len() {
                        let ((s1, e1), (s2, e2)) = (v[i], vv[j]);
                        if s1 != s2 && s1 != e2 && e1 != s2 && e1 != e2 {
                            let mut l = vec![nums[s1], nums[e1], nums[s2], nums[e2]];
                            l.sort();
                            set.insert(Tuple(l[0], l[1], l[2], l[3]));
                        }
                    }
                }
            }
        }
        set.into_iter().map(|t| vec![t.0, t.1, t.2, t.3]).collect()
    }
}

fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
}
