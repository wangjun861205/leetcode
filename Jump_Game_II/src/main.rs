struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(nums: &Vec<i32>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        let steps = nums[index] as usize;
        if index >= nums.len() - 1 {
            return 0;
        }
        if steps == 0 {
            return -1;
        }
        if index + steps >= nums.len() - 1 {
            return 1;
        }
        let l: Vec<i32> = (1..=steps)
            .into_iter()
            .map(|i| {
                if let Some(v) = cache.get(&(index + i)) {
                    *v
                } else {
                    Solution::rc(nums, index + i, cache)
                }
            })
            .filter(|v| *v != -1)
            .collect();
        if l.len() == 0 {
            cache.insert(index, -1);
            return -1;
        } else {
            let ans = l.into_iter().min().unwrap();
            cache.insert(index, ans + 1);
            return ans + 1;
        }
    }
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Solution::rc(&nums, 0, &mut cache)
    }
}
fn main() {
    println!("{}", Solution::jump(vec![1, 2, 1, 1, 1]));
}
