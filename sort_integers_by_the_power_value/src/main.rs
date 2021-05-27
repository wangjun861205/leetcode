struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn power(num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if num == 1 {
            return 0;
        }
        if cache.contains_key(&num) {
            return *cache.get(&num).unwrap();
        } else {
            let next = if num % 2 == 0 { num / 2 } else { num * 3 + 1 };
            let ans = Solution::power(next, cache);
            cache.insert(num, ans + 1);
            return ans + 1;
        }
    }
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let mut l: Vec<(i32, i32)> = (lo..=hi)
            .into_iter()
            .map(|v| (Solution::power(v, &mut cache), v))
            .collect();
        l.sort();
        l[k as usize - 1].1
    }
}
fn main() {
    println!("{}", Solution::get_kth(12, 15, 2));
}
