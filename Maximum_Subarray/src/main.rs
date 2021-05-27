struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut run_max = nums[0];
        for &v in nums[1..].into_iter() {
            run_max = v.max(run_max + v);
            max = max.max(run_max);
        }
        max
    }
}
fn main() {
    println!("Hello, world!");
}
