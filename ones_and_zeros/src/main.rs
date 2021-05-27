struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(
        mut l: Vec<(i32, i32)>,
        cache: &mut HashMap<(usize, i32, i32), i32>,
        m: i32,
        n: i32,
    ) -> i32 {
        if l.len() == 0 {
            return 0;
        }
        let (mm, nn) = l.remove(0);
        if m - mm < 0 || n - nn < 0 {
            if let Some(c) = cache.get(&(l.len(), m, n)).cloned() {
                cache.insert((l.len() + 1, m, n), c);
                return c;
            } else {
                let ans = Solution::rc(l.clone(), cache, m, n);
                cache.insert((l.len() + 1, m, n), ans);
                return ans;
            }
        } else {
            let pass = if let Some(c) = cache.get(&(l.len(), m, n)) {
                *c
            } else {
                Solution::rc(l.clone(), cache, m, n)
            };
            let pick = if let Some(c) = cache.get(&(l.len(), m - mm, n - nn)) {
                *c
            } else {
                Solution::rc(l.clone(), cache, m - mm, n - nn) + 1
            };
            let ans = pass.max(pick);
            cache.insert((l.len() + 1, m, n), ans);
            ans
        }
    }
    pub fn find_max_form(strs: Vec<String>, mut m: i32, mut n: i32) -> i32 {
        let mut l: Vec<(i32, i32)> = strs
            .into_iter()
            .map(|s| {
                let mut zeros = 0;
                let mut ones = 0;
                for c in s.chars() {
                    if c == '0' {
                        zeros += 1;
                    } else {
                        ones += 1;
                    }
                }
                (zeros, ones)
            })
            .collect();
        let mut cache = HashMap::new();
        Solution::rc(l, &mut cache, m, n)
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_max_form(
            vec!["10", "0", "1"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
            1,
            1
        )
    );
}
