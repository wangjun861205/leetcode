struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(mut bitsets: Vec<i32>, mut bitset: i32, cache: &mut HashMap<(i32, usize), i32>) -> i32 {
        if bitsets.is_empty() {
            let mut count = 0;
            for _ in 0..32 {
                count += (bitset & 1);
                bitset >>= 1;
            }
            return count;
        }
        let set = bitsets.remove(0);
        let pick = if bitset & set > 0 {
            if let Some(c) = cache.get(&(set, bitsets.len())) {
                *c
            } else {
                Solution::dp(bitsets.clone(), set, cache)
            }
        } else {
            if let Some(c) = cache.get(&(bitset | set, bitsets.len())) {
                *c
            } else {
                Solution::dp(bitsets.clone(), bitset | set, cache)
            }
        };
        let pass = if let Some(c) = cache.get(&(bitset, bitsets.len())) {
            *c
        } else {
            Solution::dp(bitsets.clone(), bitset, cache)
        };
        let ans = pick.max(pass);
        cache.insert((bitset, bitsets.len()), ans);
        ans
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let bitsets: Vec<i32> = arr
            .into_iter()
            .map(|s| {
                let mut set = 0;
                for c in s.chars() {
                    if set & 1 << (c as usize - 97) > 0 {
                        return -1;
                    }
                    set |= 1 << (c as usize - 97);
                }
                set
            })
            .filter(|v| v > &0)
            .collect();
        let mut cache = HashMap::new();
        Solution::dp(bitsets, 0, &mut cache)
    }
}
fn main() {
    println!("{}", Solution::max_length(vec!["yy", "bkhwmpbiisbldzknpm"].into_iter().map(str::to_owned).collect()));
}
