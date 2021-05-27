use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn can_reach_target(set: &mut HashSet<i32>, target: i32, nums: Vec<i32>) -> bool {
        for (i, num) in nums.iter().enumerate() {
            if num < &target {
                let next_target = target - num;
                if set.contains(&next_target) {
                    continue;
                }
                let mut remain_nums = nums.clone();
                remain_nums.remove(i);
                if Solution::can_reach_target(set, target - num, remain_nums) {
                    return true;
                }
                set.insert(next_target);
            } else if num == &target {
                return true;
            }
        }
        false
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let mut set: HashSet<i32> = HashSet::new();
        Solution::can_reach_target(&mut set, sum / 2, nums)
    }
}

fn main() {
    println!("{}", Solution::can_partition(vec![100; 200]));
}
