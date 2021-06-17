struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|v| Reverse(v[1]));
        let mut ans = 0;
        for b in box_types {
            ans += truck_size.min(b[0]) * b[1];
            truck_size -= truck_size.min(b[0]);
            if truck_size == 0 {
                break;
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
