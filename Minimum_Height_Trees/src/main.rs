struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = vec![0; n as usize];
        let mut total = 0;
        for e in &edges {
            counts[e[0] as usize] += 1;
            counts[e[1] as usize] += 1;
            total += 2;
        }
        loop {
            let set: HashSet<i32> = counts
                .iter()
                .enumerate()
                .filter_map(|(i, v)| if *v == 1 { Some(i as i32) } else { None })
                .collect();
            if set.len() * 2 == total {
                let idx = counts
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &v)| v)
                    .map(|(i, _)| i)
                    .unwrap();
                return vec![idx as i32];
            }
            if total == 2 {
                return set.into_iter().collect();
            }
            let (cur, remain): (Vec<Vec<i32>>, Vec<Vec<i32>>) = edges
                .into_iter()
                .partition(|v| set.contains(&v[0]) || set.contains(&v[1]));
            edges = remain;
            for e in cur {
                counts[e[0] as usize] -= 1;
                counts[e[1] as usize] -= 1;
                total -= 2;
            }
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_min_height_trees(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 6]
            ]
        )
    );
}
