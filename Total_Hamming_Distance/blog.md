The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given an integer array nums, return the sum of Hamming distances between all the pairs of the integers in nums.

Example 1:

> Input: nums = [4,14,2]  
> Output: 6

Explanation: In binary representation, the 4 is 0100, 14 is 1110, and 2 is 0010 (just
showing the four bits relevant in this case).
The answer will be:
HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.
Example 2:

> Input: nums = [4,14,4]  
> Output: 4

Constraints:

- 1 <= nums.length <= 104
- 0 <= nums[i] <= 109
- The answer for the given input will fit in a 32-bit integer.

---

给出的数字都在 32 位整数之内， 我们可以把每一位为 0 和为 1 的数字数量分别统计出来，假设为 0 的是 n 个， 那为 1 的就是 length - n 个， 那他们对整体距离的贡献就是 n \* (length -n)

---

```rust
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for i in 0..32 {
            let mut ones = 0;
            let mut zeros = 0;
            let mask = 1 << i;
            for j in 0..nums.len() {
                if nums[j] & mask > 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
            total += ones * zeros;
        }
        total
    }
}
```
