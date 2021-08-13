struct Solution;

use std::collections::HashSet;

impl Solution {
    fn check(s1: &String, s2: &String) -> bool {
        let count = s1
            .chars()
            .zip(s2.chars())
            .fold(0, |c, (c1, c2)| if c1 != c2 { c + 1 } else { c });
        count == 1
    }

    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut starts = vec![start];
        let mut visited = HashSet::new();
        let mut steps = 0;
        loop {
            let mut new_starts = Vec::new();
            for st in &starts {
                for b in &bank {
                    if Solution::check(st, b) && !visited.contains(b) {
                        if b == &end {
                            return steps + 1;
                        } else {
                            visited.insert(b.clone());
                            new_starts.push(b.clone());
                        }
                    }
                }
            }
            if new_starts.len() == 0 {
                return -1;
            }
            steps += 1;
            starts = new_starts;
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_mutation(
            "AACCGGTT".to_owned(),
            "AAACGGTA".to_owned(),
            vec!["AACCGGTA", "AACCGCTA", "AAACGGTA"]
                .into_iter()
                .map(|v| v.to_owned())
                .collect()
        )
    );
}
