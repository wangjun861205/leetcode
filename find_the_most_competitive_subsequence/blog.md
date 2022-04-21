Given an integer array nums and a positive integer k, return the most competitive subsequence of nums of size k.

An array's subsequence is a resulting sequence obtained by erasing some (possibly zero) elements from the array.

We define that a subsequence a is more competitive than a subsequence b (of the same length) if in the first position where a and b differ, subsequence a has a number less than the corresponding number in b. For example, [1,3,4] is more competitive than [1,3,5] because the first position they differ is at the final number, and 4 is less than 5.

Example 1:

> Input: nums = [3,5,2,6], k = 2  
> Output: [2,6]

Explanation: Among the set of every possible subsequence: {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]}, [2,6] is the most competitive.

Example 2:

> Input: nums = [2,4,3,3,5,4,9,6], k = 4  
> Output: [2,3,3,4]

Constraints:

- 1 <= nums.length <= 105
- 0 <= nums[i] <= 109
- 1 <= k <= nums.length

---

创建一个队列， 每次从 nums 中取出一个数， 倒序检查队列里的元素是不是大于当前的数， 如果大于则将该元素踢出队列， 然后将当前的数 push 进队尾, 也就是最终生成一个单调递增的队列, 但是要注意的是， 这样生成的队列最后很有可能 size < k， 所以为了保证最终拿到的队列的 size == k, 我们需要在每次踢出元素的时候做检查， 检查当前队列剩余的空位要小于 nums 里剩余的元素数

---

```rust
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let length = nums.len();
        let mut ans = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            if ans.is_empty() {
                ans.push(num);
                continue;
            }
            while let Some(last) = ans.pop() {
                if k as usize - ans.len() > length - i {
                    ans.push(last);
                    break;
                }
                if last <= num {
                    ans.push(last);
                    break;
                }
            }
            if ans.len() < k as usize {
                ans.push(num);
            }
        }
        ans
    }
}
```
