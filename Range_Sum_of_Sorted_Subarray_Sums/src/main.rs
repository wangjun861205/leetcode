struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut presum: Vec<i32> = nums
            .into_iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        let mut heap = BinaryHeap::new();
        for i in 1..presum.len() {
            for j in 0..i {
                heap.push(presum[i] - presum[j]);
            }
        }
        let l = heap.into_sorted_vec();
        println!("{:?}", l);
        l[left as usize - 1..right as usize]
            .into_iter()
            .fold(0, |mut s, v| {
                s += *v % 1000000007;
                s %= 1000000007;
                s
            })
    }
}

fn main() {
    println!("{}", Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4));
}
