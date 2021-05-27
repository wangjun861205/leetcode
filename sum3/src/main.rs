struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ns = nums.clone();
        ns.sort();
        let mut ans: HashSet<Vec<i32>> = HashSet::new();
        for (i, n) in ns.iter().enumerate() {
            let mut left_index = i + 1;
            let mut right_index = ns.len() - 1;
            while left_index < right_index {
                let sum = ns[left_index] + ns[right_index];
                if sum == -*n {
                    ans.insert(vec![*n, ns[left_index], ns[right_index]]);
                    left_index += 1;
                    right_index -= 1;
                } else if sum < -*n {
                    left_index += 1;
                } else {
                    right_index -= 1;
                }
            }
        }
        ans.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
