Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays.

Write an algorithm to minimize the largest sum among these m subarrays.

Example 1:

> Input: nums = [7,2,5,10,8], m = 2  
> Output: 18

Explanation:  
There are four ways to split nums into two subarrays.
The best way is to split it into [7,2,5] and [10,8],
where the largest sum among the two subarrays is only 18.

Example 2:

> Input: nums = [1,2,3,4,5], m = 2  
> Output: 9

Example 3:

> Input: nums = [1,4,4], m = 3  
> Output: 4

Constraints:

- 1 <= nums.length <= 1000
- 0 <= nums[i] <= 106
- 1 <= m <= min(50, nums.length)

---

首先我们能想到，最终答案的最大值就是整个`nums`的`sum`, 最小值则是`sum / m`和`nums`中的最大的元素相比较较大的那个。我们既然知道了取值范围，自然而然的就会想到用 binary search 的方法来查找这个值。剩下的看代码吧

---

```rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut sum = 0;
        let mut maximum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            maximum = maximum.max(nums[i]);
        }
        let mut min = (sum / m).max(maximum);
        let mut max = sum;
        while min < max {
            let mid = (min + max) / 2;
            let mut count = 0;
            let mut curr = 0;
            for i in 0..nums.len() {
                if curr + nums[i] > mid {
                    count += 1;
                    curr = nums[i];
                } else {
                    curr += nums[i];
                }
            }
            count += 1;
            if count > m {
                min = mid + 1;
            } else {
                max = mid;
            }
        }
        min
    }
}

```
