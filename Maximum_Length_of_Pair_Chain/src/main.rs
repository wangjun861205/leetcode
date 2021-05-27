struct Solution;

use std::cmp::{Ord, Ordering, PartialOrd};
#[derive(PartialEq, Eq, Clone, Debug)]
struct Pair(i32, i32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return self.1.cmp(&other.1);
        } else {
            return self.0.cmp(&other.0);
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

use std::collections::HashMap;

impl Solution {
    fn rc(pairs: &Vec<Pair>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if index == pairs.len() - 1 {
            return 1;
        }
        let p = pairs[index].clone();
        let mut max = 1;
        for (i, v) in pairs.iter().enumerate().skip(index + 1) {
            if v.0 > p.1 {
                if let Some(c) = cache.get(&i) {
                    max = max.max(*c + 1);
                } else {
                    let ans = Solution::rc(pairs, i, cache);
                    max = max.max(ans + 1);
                }
            }
        }
        cache.insert(index, max);
        max
    }

    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut ps: Vec<Pair> = pairs.into_iter().map(|v| Pair(v[0], v[1])).collect();
        ps.sort();
        println!("{:?}", ps);
        let mut max = 0;
        let mut cache = HashMap::new();
        for (i, _) in ps.iter().enumerate() {
            max = max.max(Solution::rc(&ps, i, &mut cache));
        }
        max
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_longest_chain(vec![
            vec![9, 10],
            vec![9, 10],
            vec![4, 5],
            vec![-9, -3],
            vec![-9, 1],
            vec![0, 3],
            vec![6, 10],
            vec![-5, -4],
            vec![-7, -6]
        ])
    );
}
