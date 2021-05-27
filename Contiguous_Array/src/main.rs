struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = 0_usize;
        let mut map = HashMap::new();
        map.insert(0_i32, 0_usize);
        for (i, v) in nums.into_iter().enumerate() {
            if v == 0 {
                count -= 1;
            } else {
                count += 1;
            }
            if let Some(e) = map.get(&count) {
                ans = ans.max(i - *e + 1);
            } else {
                map.insert(count, i + 1);
            }
        }
        ans as i32
    }
}
fn main() {
    println!("Hello, world!");
}
