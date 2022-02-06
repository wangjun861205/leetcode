struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut ans = 0;
        for n in nums {
            if n == 1 {
                stack.push(1);
                ans = ans.max(stack.len() as i32);
            } else {
                if k > 0 {
                    k -= 1;
                    stack.push(0);
                    ans = ans.max(stack.len() as i32);
                } else {
                    while !stack.is_empty() {
                        let v = stack.remove(0);
                        if v == 0 {
                            stack.push(0);
                            break;
                        }
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_ones(
            vec![
                1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
                1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1
            ],
            8
        )
    );
}
