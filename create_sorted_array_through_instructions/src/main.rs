struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut m: BTreeMap<i32, i32> = BTreeMap::new();
        let mut total = 0;
        for v in instructions {
            let mut lt_count = 0;
            let mut gt_count = 0;
            m.range(0..v).for_each(|(_, c)| lt_count += *c);
            m.range(v + 1..).for_each(|(_, c)| gt_count += *c);
            total += lt_count.min(gt_count);
            *m.entry(v).or_insert(0) += 1;
        }
        total
    }
}
fn main() {
    println!("{}", Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
}
