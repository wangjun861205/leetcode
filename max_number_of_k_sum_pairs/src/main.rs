struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut m: HashMap<i32, i32> = HashMap::new();
        for v in nums {
            match m.get_mut(&(k - v)) {
                Some(c) => {
                    if c > &mut 0 {
                        *c -= 1;
                        count += 1;
                    } else {
                        *m.entry(v).or_insert(0) += 1;
                    }
                }
                None => *m.entry(v).or_insert(0) += 1,
            }
        }
        count
    }
}

fn main() {
    println!("{}", Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
}
