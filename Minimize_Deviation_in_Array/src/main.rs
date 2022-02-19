struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut ans = i32::MAX;
        let mut heap = BinaryHeap::new();
        for n in nums {
            let n = if n % 2 == 1 { n * 2 } else { n };
            heap.push(n);
            min = min.min(n);
        }
        while let Some(n) = heap.pop() {
            ans = ans.min(n - min);
            if n % 2 == 1 {
                break;
            }
            let n = n / 2;
            min = min.min(n);
            heap.push(n);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::minimum_deviation(vec![3, 5]));
}
