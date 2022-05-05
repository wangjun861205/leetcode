struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(mut nums: Vec<i32>) -> i32 {
        let mut products: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let prod = nums[i] * nums[j];
                count += *products.get(&prod).unwrap_or(&0);
                *products.entry(prod).or_insert(0) += 1;
            }
        }
        count * 8
    }
}

fn main() {
    println!("{}", Solution::tuple_same_product(vec![1, 2, 4, 5, 10]));
}
