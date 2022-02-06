struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            let mut ans = 0;
            for j in 1..=(k as usize).min(i + 1) {
                let max = *arr[i + 1 - j..=i].iter().max().unwrap();
                ans = ans.max(dp[i + 1 - j] + max * j as i32);
            }
            dp[i + 1] = ans;
        }

        *dp.last().unwrap()
    }
}
fn main() {
    // println!("{}", Solution::max_sum_after_partitioning(vec![1, 15], 3));
    println!(
        "{}",
        Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4)
    );
}
