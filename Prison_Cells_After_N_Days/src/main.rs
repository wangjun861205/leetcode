struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let s = cells
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("");
        let mut cache: HashMap<u8, i32> = HashMap::new();
        let mut num = u8::from_str_radix(&s, 2).unwrap();
        let mut i = n;
        while i > 0 {
            i -= 1;
            let left_shift = num << 1;
            let right_shift = num >> 1;
            let next_num = (left_shift ^ right_shift ^ 0b11111111) & 0b01111110;
            if let Some(c) = cache.get(&next_num) {
                i %= *c - i;
            } else {
                cache.insert(next_num, i);
            }
            num = next_num;
        }
        format!("{:08b}", num)
            .chars()
            .map(|v| v.to_string().parse::<i32>().unwrap())
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000)
    );
}
