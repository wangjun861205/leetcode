struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(mut a: Vec<i32>, b: Vec<i32>, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        let n = a.remove(0);
        let indices: Vec<usize> = b
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == n)
            .map(|(i, _)| i)
            .collect();
        let mut ans = 0;
        if let Some(c) = cache.get(&(a.len(), b.len())) {
            ans = *c;
        } else {
            ans = ans.max(Solution::rc(a.clone(), b.clone(), cache));
        }
        for i in indices {
            if let Some(&c) = cache.get(&(a.len(), b[i + 1..].len())) {
                ans = ans.max(c + 1);
            } else {
                ans = ans.max(Solution::rc(a.clone(), b[i + 1..].to_vec(), cache) + 1);
            }
        }
        cache.insert((a.len() + 1, b.len()), ans);
        ans
    }
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        let mut cache = HashMap::new();
        Solution::rc(a, b, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1])
    );
}
