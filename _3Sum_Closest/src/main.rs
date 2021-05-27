struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = i32::max_value();
        nums.sort();
        for i in 0..nums.len() {
            let mut lo = i + 1;
            let mut hi = nums.len() - 1;
            let t = target - nums[i];
            while lo < hi {
                if (t - nums[lo] - nums[hi]).abs() < diff.abs() {
                    diff = t - nums[lo] - nums[hi];
                }

                if nums[lo] + nums[hi] == t {
                    break;
                }
                if nums[lo] + nums[hi] < t {
                    lo += 1;
                } else if nums[lo] + nums[hi] > t {
                    hi -= 1;
                }
            }
        }
        target - diff
    }
}
fn main() {
    println!("Hello, world!");
}
