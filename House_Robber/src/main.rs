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
        let ans = (nums[index] + rob).max(skip);
        cache[index] = ans;
        ans
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut cache = vec![-1; nums.len()];
        Solution::rc(&nums, 0, &mut cache)
    }
}
fn main() {
    println!("Hello, world!");
}
