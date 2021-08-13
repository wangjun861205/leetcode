struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max(piles: &Vec<i32>, start: usize, end: usize, cache: &mut HashMap<(usize, usize), i32>, alex: &mut i32, lee: &mut i32) {
        if end - start == 1 {
            *alex += piles[0].max(piles[1]);
            *lee += piles[0].min(piles[1]);
            return;
        }
        let left = if let Some(c) = cache.get(&(start + 1, end)) {
            piles[start] + *c
        } else {
            let mut a = 0;
            let mut l = 0;
            Solution::max(piles, start + 1, end, cache, &mut a, &mut l);
            piles[start] + a.max(l)
        };
        let right = if let Some(c) = cache.get(&(start, end - 1)) {
            piles[end] + *c
        } else {
            let mut a = 0;
            let mut l = 0;
            Solution::max(piles, start, end - 1, cache, &mut a, &mut l);
            piles[end] + a.max(l)
        };
        let result = left.max(right);
        cache.insert((start, end), result);
        if (end - start) % 2 == 1 {
            *alex += result;
        } else {
            *lee += result;
        }
    }

    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut cache = HashMap::new();
        let mut alex = 0;
        let mut lee = 0;
        Solution::max(&piles, 0, piles.len() - 1, &mut cache, &mut alex, &mut lee);
        alex > lee
    }
}

fn main() {
    println!("{}", Solution::stone_game(vec![5, 3, 4, 5]));
}
