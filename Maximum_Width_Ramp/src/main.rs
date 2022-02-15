struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = vec![(nums[0], 0_usize)];
        let mut ans = 0;
        for (i, n) in nums.into_iter().enumerate().skip(1) {
            let (last_val, _) = *stack.last().unwrap();
            if n < last_val {
                stack.push((n, i));
                continue;
            } else {
                let mut l = 0_usize;
                let mut r = stack.len() - 1;
                while l < r {
                    let m = (l + r) / 2;
                    let (val, _) = stack[m];
                    if val > n {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                ans = ans.max(i - stack[l].1);
            }
        }
        ans as i32
    }
}
fn main() {
    println!("{}", Solution::max_width_ramp(vec![1, 2, 1]));
}
