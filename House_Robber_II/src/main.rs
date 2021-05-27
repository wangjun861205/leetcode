struct Solution;

impl Solution {
    fn rc(nums: &Vec<i32>, index: usize, cache: &mut Vec<i32>) -> i32 {
        if index >= nums.len() {
            return 0;
        }
        let rob = if let Some(c) = cache.get(index + 2) {
            if c != &-1 {
                *c
            } else {
                Solution::rc(nums, index + 2, cache)
            }
        } else {
            Solution::rc(nums, index + 2, cache)
        };
        let skip = if let Some(c) = cache.get(index + 1) {
            if c != &-1 {
                *c
            } else {
                Solution::rc(nums, index + 1, cache)
            }
        } else {
            Solution::rc(nums, index + 1, cache)
        };
        let ans = (rob + nums[index]).max(skip);
        cache[index] = ans;
        ans
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            _ => {
                let mut rob_cache = vec![-1; nums.len() - 3];
                let rob =
                    nums[0] + Solution::rc(&nums[2..nums.len() - 1].to_vec(), 0, &mut rob_cache);
                let mut skip_cache = vec![-1; nums.len() - 1];
                let skip = Solution::rc(&nums[1..].to_vec(), 0, &mut skip_cache);
                rob.max(skip)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
