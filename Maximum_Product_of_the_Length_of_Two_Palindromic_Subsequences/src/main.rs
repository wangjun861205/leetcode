struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp<'a>(chars: &'a [char], cache: &mut HashMap<&'a [char], i32>) -> i32 {
        if chars.len() == 0 {
            return 0;
        }
        if chars.len() == 1 {
            return 1;
        }
        let pick = {
            if let Some(i) = chars.iter().rposition(|v| v == &chars[0]) {
                if i == 0 {
                    1
                } else {
                    if let Some(c) = cache.get(&chars[1..i]) {
                        *c + 2
                    } else {
                        Solution::dp(&chars[1..i], cache) + 2
                    }
                }
            } else {
                1
            }
        };
        let ignore = if let Some(c) = cache.get(&chars[1..chars.len()]) {
            *c
        } else {
            Solution::dp(&chars[1..chars.len()], cache)
        };
        let ans = pick.max(ignore);
        cache.insert(chars, ans);
        ans
    }
    pub fn max_product(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut bitmask = 1;
        let mut ans = 0;
        while bitmask < 2i32.pow(chars.len() as u32) {
            let mut ones = Vec::new();
            let mut zeros = Vec::new();
            let mut mask = bitmask;
            for i in 0..chars.len() {
                if mask & 1 == 1 {
                    ones.push(chars[i]);
                } else {
                    zeros.push(chars[i]);
                }
                mask >>= 1;
            }
            bitmask += 1;
            let one_count = Solution::dp(&ones, &mut HashMap::new());
            let zero_count = Solution::dp(&zeros, &mut HashMap::new());
            ans = ans.max(one_count * zero_count);
        }
        ans
    }
}

// ctpipc
// ceec
// cpipc

fn main() {
    println!("{}", Solution::max_product("leetcodecom".into()));
}
