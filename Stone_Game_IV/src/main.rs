struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dfs(n: i32, cache: &mut HashMap<i32, bool>) -> bool {
        if n == 0 {
            return false;
        }
        for i in 1..=(n as f32).sqrt() as i32 {
            let next = if let Some(c) = cache.get(&(n - i * i)) {
                *c
            } else {
                Solution::dfs(n - i * i, cache)
            };
            if !next {
                cache.insert(n, true);
                return true;
            }
        }
        cache.insert(n, false);
        false
    }
    pub fn winner_square_game(n: i32) -> bool {
        Solution::dfs(n, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::winner_square_game(92719));
}
