struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut left_sum: Vec<i32> = nums
            .iter()
            .scan(0, |s, v| {
                *s += *v;
                Some(*s)
            })
            .collect();
        if left_sum.last().unwrap() < &x {
            return -1;
        }
        left_sum.insert(0, 0);
        let mut right_sum: HashMap<i32, usize> = nums
            .iter()
            .rev()
            .enumerate()
            .scan(0, |s, (i, v)| {
                *s += *v;
                Some((*s, i + 1))
            })
            .collect();
        right_sum.insert(0, 0);
        let mut ans = -1;
        for (i, l) in left_sum.into_iter().enumerate() {
            if l > x {
                return ans;
            }
            if let Some(r) = right_sum.get(&(x - l)) {
                if ans == -1 {
                    ans = (i + r) as i32;
                } else {
                    ans = ans.min((i + r) as i32);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10));
}
