Given an array nums of distinct positive integers, return the number of tuples (a, b, c, d) such that a _ b = c _ d where a, b, c, and d are elements of nums, and a != b != c != d.

Example 1:

> Input: nums = [2,3,4,6]  
> Output: 8

Explanation: There are 8 valid tuples:
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)

Example 2:

> Input: nums = [1,2,4,5,10]  
> Output: 16

Explanation: There are 16 valid tuples:
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)

Constraints:

- 1 <= nums.length <= 1000
- 1 <= nums[i] <= 104
- All elements in nums are distinct.

---

一开始看这题首先想到的是如何去重的问题， 后来想到这里面的数字都是不重复的，所以找任意两个数字计算乘积， 其他任意两个具有相同乘积的数字一定是不相同的， 就是 a _ b = p, c _ d = p, 只要 a != c 那 b != d。剩下的事就是用一个 HashMap 来存储每个乘积下所有 pair 的数量， 这里要注意的是，最终是要 tuple 的数量， 4 个数字一共可以组成 8 个 tuple(左侧 pair 交换 2 种情况， 右侧 pair 交换 2 种情况， 整体左右交换 2 种情况， 2 _ 2 _ 2)。

---

```rust
use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(mut nums: Vec<i32>) -> i32 {
        let mut products: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let prod = nums[i] * nums[j];
                count += *products.get(&prod).unwrap_or(&0);
                *products.entry(prod).or_insert(0) += 1;
            }
        }
        count * 8
    }
}
```
