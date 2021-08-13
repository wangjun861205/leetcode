struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(nums: &Vec<(i32, i32)>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        let len = nums.len();
        if index == len {
            return 0;
        }
        if index == len - 1 {
            return nums[len - 1].1;
        }
        if nums[index].0 != nums[index + 1].0 - 1 {
            return nums[index].1
                + if let Some(c) = cache.get(&(index + 1)) {
                    *c
                } else {
                    Solution::dp(nums, index + 1, cache)
                };
        }
        let using = nums[index].1 + {
            if let Some(c) = cache.get(&(index + 2)) {
                *c
            } else {
                Solution::dp(nums, index + 2, cache)
            }
        };
        let avoid = if let Some(c) = cache.get(&(index + 1)) {
            *c
        } else {
            Solution::dp(nums, index + 1, cache)
        };
        let result = using.max(avoid);
        cache.insert(index, result);
        result
    }

    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let list = nums
            .into_iter()
            .fold(Vec::new(), |mut l: Vec<(i32, i32)>, v| {
                if l.is_empty() {
                    l.push((v, v));
                } else {
                    let last = l.last().unwrap().clone();
                    if last.0 == v {
                        l.last_mut().unwrap().1 += v;
                    } else {
                        l.push((v, v));
                    }
                }
                l
            });
        let mut cache = HashMap::new();

        Solution::dp(&list, 0, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::delete_and_earn(vec![8, 7, 3, 8, 1, 4, 10, 10, 10, 2])
    );
}
