struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(nums: Vec<i32>, ans: &mut HashSet<Vec<i32>>) {
        if nums.len() == 1 {
            ans.insert(nums);
            return;
        }
        for i in 0..nums.len() {
            let mut remain = nums.clone();
            remain.remove(i);
            ans.insert(remain.clone());
            Solution::rc(remain, ans)
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        Solution::rc(nums.clone(), &mut ans);
        ans.insert(vec![]);
        ans.insert(nums.clone());
        ans.into_iter().collect()
    }
}
fn main() {
    println!("Hello, world!");
}
