struct Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let sum1: i32 = nums.iter().step_by(2).sum();
        let sum2: i32 =
            nums.iter().skip(1).step_by(2).sum::<i32>() - nums[nums.len() - 1] + nums[0];
        sum1.max(sum2)
    }
}

fn main() {
    println!("{}", Solution::array_pair_sum(vec![]));
}
