struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(nums: Vec<i32>, result: &mut HashSet<Vec<i32>>) {
        if nums.len() == 1 {
            result.insert(nums);
            return;
        }
        for i in 0..nums.len() {
            let mut ns = nums.clone();
            ns.remove(i);
            ns.sort();
            result.insert(ns.clone());
            Solution::rc(ns, result);
        }
    }
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        Solution::rc(nums.clone(), &mut result);
        result.insert(vec![]);
        result.insert(nums);
        result.into_iter().collect()
    }
}
fn main() {
    println!("Hello, world!");
}
