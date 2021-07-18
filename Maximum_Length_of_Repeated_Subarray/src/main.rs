struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
        let mut ans = 0;
        for i in (0..nums1.len()).rev() {
            for j in (0..nums2.len()).rev() {
                if nums1[i] == nums2[j] {
                    if i == nums1.len() - 1 || j == nums2.len() - 1 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i + 1][j + 1] + 1;
                    }
                    ans = ans.max(dp[i][j])
                }
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );
}
