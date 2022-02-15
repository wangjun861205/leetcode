struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(piles: Vec<i32>, m: usize, is_alice: bool, cache: &mut HashMap<(usize, usize, bool), i32>) -> i32 {
        let mut ans = if is_alice { 0 } else { i32::MAX };
        if 2 * m >= piles.len() {
            if is_alice {
                return piles.into_iter().sum::<i32>();
            }
            return 0;
        }
        for i in 1..2 * m + 1 {
            if i >= piles.len() {
                break;
            } else {
                let current = piles.clone();
                let (left, right) = current.split_at(i);
                let sum = left.into_iter().sum::<i32>();
                let remain = right.to_vec();
                let next = if let Some(c) = cache.get(&(remain.len(), i.max(m), !is_alice)) {
                    *c
                } else {
                    Solution::dp(remain, i.max(m), !is_alice, cache)
                };
                if is_alice {
                    ans = ans.max(sum + next);
                } else {
                    ans = ans.min(next);
                }
            }
        }
        cache.insert((piles.len(), m, is_alice), ans);
        ans
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Solution::dp(piles, 1, true, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
}
