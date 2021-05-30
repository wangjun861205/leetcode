struct Solution;

use std::i32;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]) - nums[0].min(nums[1]);
        }
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        if max == min {
            return 0;
        }
        let gap = ((max - min) / (nums.len() - 1) as i32).max(1);
        let mut buckets: Vec<(i32, i32)> =
            vec![(i32::MAX, i32::MIN); (max - min) as usize / gap as usize + 1];
        for n in nums {
            let idx = ((n - min) / gap) as usize;
            if n < buckets[idx].0 {
                buckets[idx].0 = n;
            }
            if n > buckets[idx].1 {
                buckets[idx].1 = n;
            }
        }
        buckets = buckets
            .into_iter()
            .filter(|b| b.0 != i32::MAX && b.1 != i32::MIN)
            .collect();
        buckets
            .windows(2)
            .max_by_key(|&w| w[1].0 - w[0].1)
            .map(|w| w[1].0 - w[0].1)
            .unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::maximum_gap(vec![1, 1, 1, 1, 1, 5, 5, 5, 5, 5])
    );
}
