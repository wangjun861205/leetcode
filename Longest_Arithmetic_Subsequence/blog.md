Given an array nums of integers, return the length of the longest arithmetic subsequence in nums.

Recall that a subsequence of an array nums is a list nums[i1], nums[i2], ..., nums[ik] with 0 <= i1 < i2 < ... < ik <= nums.length - 1, and that a sequence seq is arithmetic if seq[i+1] - seq[i] are all the same value (for 0 <= i < seq.length - 1).

Example 1:

Input: nums = [3,6,9,12]  
Output: 4

Explanation:

The whole array is an arithmetic sequence with steps of length = 3.

Example 2:

Input: nums = [9,4,7,2,10]  
Output: 3

Explanation:

The longest arithmetic subsequence is [4,7,10].

Example 3:

Input: nums = [20,1,15,3,10,5,8]  
Output: 4

Explanation:

The longest arithmetic subsequence is [20,15,10,5].

Constraints:

- 2 <= nums.length <= 1000
- 0 <= nums[i] <= 500

---

假设 dp(i)(k)是 nums[i]之后的 nums[j]-nums[i] == k 的元素数量， 则  
dp(i)(k) = dp(j)(k) + 1, k = nums[j] - nums[j], i < j < nums.length

我们只需要在计算的过程中找到 dp(i)(k)的最大值就好了

---

代码实现(Rust):

```rust
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = vec![vec![0; 1001]; 1000];
        for i in (0..nums.len() - 1).rev() {
            for j in (i + 1..nums.len()).rev() {
                dp[i][(nums[j] - nums[i] + 500) as usize] =
                    dp[j][(nums[j] - nums[i] + 500) as usize] + 1;
                ans = ans.max(dp[i][(nums[j] - nums[i] + 500) as usize]);
            }
        }
        ans + 1
    }
}
```
