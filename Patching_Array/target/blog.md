Given a sorted integer array nums and an integer n, add/patch elements to the array such that any number in the range [1, n] inclusive can be formed by the sum of some elements in the array.

Return the minimum number of patches required.

Example 1:

Input: nums = [1,3], n = 6  
Output: 1

Explanation:  
Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
So we only need 1 patch.

Example 2:

Input: nums = [1,5,10], n = 20  
Output: 2

Explanation: The two patches can be [2, 4].

Example 3:

Input: nums = [1,2,2], n = 5  
Output: 0

Constraints:

- 1 <= nums.length <= 1000
- 1 <= nums[i] <= 104
- nums is sorted in ascending order.
- 1 <= n <= 231 - 1

---

数学很美，以前是我有眼无珠

这题从昨天晚上一直想到今天早上，后来还是题目本身提醒了我， n 的取值范围是 uint32，这直接告诉我们别在它身上打主意了，要不就算有超算的能力也白搭。

在纸上划拉了划拉，还真让我给划拉出来了。

> nums = [1]  
> convered range = [1]

> nums = [1, 2]
> convered range = [1, 2, 3]

> nums = [1, 2, 4]
> convered range = [1, 2, 3, 4, 5, 6]

> nums = [1, 2, 5]
> convered range = [1, 2, 3, 5, 6, 7, 8]

要想无缝覆盖的 range， 我们需要让新加入的 num <= 当前的 sum

> nums = [1, 2]  
> convered range = [1, 2, 3]
> sum = 3  
> 现在我们如果想覆盖 1 到 4， 我们可以添加 1 到 nums, 如果想覆盖 1..5, 我们可以添加 2 进去， 其实这个 sum 表达的是当前 nums 的所能无缝覆盖的最大值。这说白了就是一个基于当前覆盖范围来扩大覆盖范围的问题。那什么时候会出现 gap 呢？新添加的值大于当前覆盖最大值+1 的时候。这时候我们要想无缝覆盖，就得把中间的这个 gap 填平。

表达的不好，不会严谨的证明，只能照着感觉说， 看不明白的，可以自己推演一下

---

代码实现(Rust):

```rust

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0_i64;
        for v in nums {
            if sum >= n as i64 {
                break;
            }
            if v as i64 <= sum + 1 {
                sum += v as i64;
            } else {
                while sum + 1 < v as i64 {
                    sum += sum + 1;
                    ans += 1;
                    if sum >= n as i64 {
                        return ans;
                    }
                }
                sum += v as i64;
            }
        }
        while sum < n as i64 {
            sum += sum + 1;
            ans += 1;
        }
        ans
    }
}

```
