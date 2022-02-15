struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut zero_count = 0;
        let mut one_count = 0;
        for v in &nums {
            if v == &0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        if zero_count == 0 || one_count == 0 {
            return 0;
        }
        let mut zero_stack = nums[..zero_count].to_vec();
        let mut ones = zero_stack.iter().filter(|v| v == &&1).count();
        let mut zero_steps = ones;
        for v in nums[zero_count..].into_iter() {
            if zero_stack.remove(0) == 1 {
                ones -= 1;
            }
            if v == &1 {
                ones += 1;
            }
            zero_stack.push(*v);
            zero_steps = zero_steps.min(ones);
        }

        let mut one_stack = nums[..one_count].to_vec();
        let mut zeros = one_stack.iter().filter(|v| v == &&0).count();
        let mut one_steps = zeros;
        for v in nums[one_count..].into_iter() {
            if one_stack.remove(0) == 0 {
                zeros -= 1;
            }
            if v == &0 {
                zeros += 1;
            }
            one_stack.push(*v);
            one_steps = one_steps.min(zeros);
        }

        one_steps.min(zero_steps) as i32
    }
}

fn main() {
    println!("{}", Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]));
}
