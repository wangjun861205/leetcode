struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        // 因为只用到dp[i-1]的值， 所以只需要保存一个值就可以
        let mut dp = 0;
        let mut ans = 0;
        // 上一个>right的元素的索引值
        let mut prev = -1;
        for (i, n) in nums.into_iter().enumerate() {
            if n < left {
                ans += dp;
            } else if n > right {
                dp = 0;
                prev = i as i32;
            } else {
                dp = i as i32 - prev;
                ans += dp;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3));
}
