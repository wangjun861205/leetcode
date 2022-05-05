Example 1:

> Input: nums = [2,6,4,8,10,9,15]  
> Output: 5

Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.

Example 2:

> Input: nums = [1,2,3,4]  
> Output: 0

Example 3:

> Input: nums = [1]  
> Output: 0

Constraints:

- 1 <= nums.length <= 104
- -105 <= nums[i] <= 105

Follow up: Can you solve it in O(n) time complexity?

---

一开始觉得这题挺简单， 结果被它生生折磨了一下午。  
最终的思路是分别正向和反向的遍历数组，各找出第一个违反规则的元素， 并记录下它们的下标， 最终两个下标的差值就是需要重新排序的最短连续子数组。
详细点说就是， 从左向右， 如果某一元素大于其左侧所有元素的最大值，我们就可以认为这个元素违反了排序规则， 从右向左， 如果某一元素小于其右侧所有元素的最小值， 我们就可以认为这个元素违反了排序规则。

---

```rust
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let prefix_sum: Vec<bool> = nums
            .iter()
            .scan(i32::MIN, |max, v| {
                *max = *max.max(&mut v.clone());
                Some(v < max)
            })
            .collect();
        let mut rev_prefix_sum: Vec<bool> = nums
            .iter()
            .rev()
            .scan(i32::MAX, |min, v| {
                *min = *min.min(&mut v.clone());
                Some(v > min)
            })
            .collect();
        rev_prefix_sum.reverse();
        if let Some(left) = rev_prefix_sum.into_iter().position(|v| v) {
            if let Some(right) = prefix_sum.into_iter().rposition(|v| v) {
                return (right - left + 1) as i32;
            }
            return 0;
        }
        0
    }
}
```
