Given an integer array arr and an integer difference, return the length of the longest subsequence in arr which is an arithmetic sequence such that the difference between adjacent elements in the subsequence equals difference.

A subsequence is a sequence that can be derived from arr by deleting some or no elements without changing the order of the remaining elements.

Example 1:

> Input: arr = [1,2,3,4], difference = 1  
> Output: 4

Explanation: The longest arithmetic subsequence is [1,2,3,4].

Example 2:

> Input: arr = [1,3,5,7], difference = 1  
> Output: 1

Explanation: The longest arithmetic subsequence is any single element.

Example 3:

> Input: arr = [1,5,7,8,5,3,4,2,1], difference = -2  
> Output: 4

Explanation: The longest arithmetic subsequence is [7,5,3,1].

Constraints:

- 1 <= arr.length <= 105
- -104 <= arr[i], difference <= 104

---

假设 dp(v)是到目前为止，以 v 结尾的 subsequence 的长度, 则 dp(v) = dp(v-difference) + 1.

---

代码实现(Rust):

```rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut ans = 1;
        for v in arr {
            if let Some(&c) = counts.get(&(v - difference)) {
                *counts.entry(v).or_insert(0) = c + 1;
                ans = ans.max(c + 1);
            } else {
                counts.insert(v, 1);
            }
        }
        ans
    }
}
```
