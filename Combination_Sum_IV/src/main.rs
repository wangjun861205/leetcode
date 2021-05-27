struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(nums: &Vec<i32>, target: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        let mut ans = 0;
        for v in nums {
            let sub = target - *v;
            if sub == 0 {
                ans += 1;
                continue;
            }
            if sub > 0 {
                if let Some(c) = cache.get(&sub) {
                    ans += *c;
                } else {
                    ans += Solution::rc(nums, target - *v, cache);
                }
            }
        }
        cache.insert(target, ans);
        ans
    }
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache = HashMap::new();
        Solution::rc(&nums, target, &mut cache)
    }
}
fn main() {
    println!("{}", Solution::combination_sum4(vec![1, 2, 3], 4));
}
