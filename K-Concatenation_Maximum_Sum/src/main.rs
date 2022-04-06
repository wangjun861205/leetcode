struct Solution;

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let sum = arr.iter().map(|v| *v as i128).sum::<i128>();
        let mut prefix_max = 0i128;
        let mut prefix_sum = 0i128;
        for i in 0..arr.len() {
            prefix_sum += arr[i] as i128;
            prefix_max = prefix_max.max(prefix_sum);
        }
        let mut suffix_max = 0i128;
        let mut suffix_sum = 0i128;
        for i in (0..arr.len()).rev() {
            suffix_sum += arr[i] as i128;
            suffix_max = suffix_max.max(suffix_sum);
        }
        let mut mid_max = 0;
        let mut dp = vec![0; arr.len()];
        for i in 0..arr.len() {
            if i == 0 {
                dp[i] = 0.max(arr[i] as i128);
            } else {
                dp[i] = (dp[i - 1] + arr[i] as i128).max(arr[i] as i128);
            }
            mid_max = mid_max.max(dp[i]);
        }
        if k == 1 {
            (sum.max(prefix_max).max(suffix_max).max(mid_max) % (10i128.pow(9) + 7)) as i32
        } else {
            ((prefix_max + suffix_max + sum * (k as i128 - 2))
                .max(prefix_max + suffix_max)
                .max(mid_max)
                % (10i128.pow(9) + 7)) as i32
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::k_concatenation_max_sum(vec![-5, 4, -4, -3, 5, -3], 3)
    );
}

// -5, -2,  0,  0,  3, 9, -2, -5, 4, -5, -2,  0,  0,  3, 9, -2, -5, 4, -5, -2,  0,  0, 3, 9, -2, -5, 4
// -5, -7, -7, -7, -4, 5,  3, -2, 2, -3, -5, -5, -5, -2, 7,  5,  0, 4, -1, -3, -3, -3, 0, 9,  7,  2, 6
