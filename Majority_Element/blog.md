Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

Example 1:

> Input: nums = [3,2,3]  
> Output: 3

Example 2:

> Input: nums = [2,2,1,1,1,2,2]  
> Output: 2

Constraints:

- n == nums.length
- 1 <= n <= 5 \* 104
- -231 <= nums[i] <= 231 - 1

Follow-up: Could you solve the problem in linear time and in O(1) space?

---

这题如果根据 Follow-up 的要求来做，会涉及到一个很神奇的算法，Boyer–Moore majority vote algorithm, 这个算法**仅**适用于一个集合中**一定**存在**多数元素**的情况， 即某一种元素的出现次数大于等于集合元素数量的一半。

算法的核心是维持一个 major 保存多数元素， 一个 count 保存计数， 遍历元素集合， 把当前元素 curr 与 major 进行对比， 如果相同则 count++，否则 count--, 当 count==0 时， major=curr

---

```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut curr = nums[0];
        for v in nums.into_iter().skip(1) {
            if count == 0 {
                count = 1;
                curr = v;
            } else {
                if v == curr {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }
        curr
    }
}
```
