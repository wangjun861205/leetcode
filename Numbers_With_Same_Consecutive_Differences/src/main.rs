struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(n: i32, k: i32, prev: i32, cache: &mut HashMap<(i32, i32, i32), Vec<i32>>) -> Vec<i32> {
        let mut cur = Vec::new();
        if prev + k < 10 {
            cur.push(prev + k);
        }
        if prev - k >= 0 {
            cur.push(prev - k);
        }
        if cur.len() == 2 && cur[0] == cur[1] {
            cur.pop();
        }
        if n == 1 {
            return cur;
        }
        let mut ans = Vec::new();
        for v in cur {
            let nexts = if let Some(c) = cache.get(&(n - 1, k, v)) {
                c.clone()
            } else {
                Solution::dp(n - 1, k, v, cache)
            };
            for next in nexts {
                ans.push(v * 10_i32.pow(n as u32 - 1) + next);
            }
        }
        cache.insert((n, k, prev), ans.clone());
        ans
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut cache = HashMap::new();
        for i in 1..10 {
            let nexts = Solution::dp(n - 1, k, i, &mut cache);
            for next in nexts {
                ans.push(i * 10_i32.pow(n as u32 - 1) + next);
            }
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::nums_same_consec_diff(2, 0));
}
