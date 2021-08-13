You are given an integer array nums and an integer k.

For each index i where 0 <= i < nums.length, change nums[i] to be either nums[i] + k or nums[i] - k.

The score of nums is the difference between the maximum and minimum elements in nums.

Return the minimum score of nums after changing the values at each index.

Example 1:

> Input: nums = [1], k = 0  
> Output: 0  
> Explanation: The score is max(nums) - min(nums) = 1 - 1 = 0.

Example 2:

> Input: nums = [0,10], k = 2  
> Output: 6  
> Explanation: Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.

Example 3:

> Input: nums = [1,3,6], k = 3  
> Output: 3  
> Explanation: Change nums to be [4, 6, 3]. The score is max(nums) - min(nums) = 6 - 3 = 3.

Constraints:

- 1 <= nums.length <= 104
- 0 <= nums[i] <= 104
- 0 <= k <= 104

---

这题前段时间尝试做过，费了半天劲没搞出来，最后评论区的答案也赌气没看就放弃了。刚才随机抽题，又抽到这一道，瞬间觉得被侮辱了，大爷的，再搞它。

首先说这题的难度没有标错，最后做完确实感觉给个 medium 都有点勉强，给个 esay 也说的过去。重点是，思考问题的时候别想的太复杂。别因为它是要求最小值，上来就 dp、greedy 伺候，这样一准翻车。

解这题最好的方法就是读完题，闭上眼，想象一个平面直角坐标系，然后在脑子里移动坐标系上的各个点。整个思考的过程有 3 个关键点:

1. 排序, 这个我想大家都能想到，虽然不知道排序是为了什么，但是本能让我们觉得排序之后会更容易解决这一题。
2. 转折点, 排完序之后的图里面一定会存在一个转折点，这个转折点之前的数字都减去 k, 而转折点之后的数字都加上 k, 这样得到的差值是最小的。(这个咋想到的？闭上眼，从脑子里想吧， 开始你可能会想到这个点在数组的中心，头尾两端分别加 k 和减 k 就可以， 但是你把脑子里的图变的稍微极端一点，你会发现这样不成立，然后你就会想到，不管这个点在哪，反正这个点一定是存在, 我们的任务其实就是找到这个点)
3. 分段极值， 假设这个点 i 我们找到了, 我们得到 nums[0] + k, nums[1] + k, ..., nums[i] + k, nums[i+1] - k, nums[i+2] - k, ..., nums[n-1] - k, 然后问题来了， 我们怎么取这里面的 min 和 max?遍历一遍?肯定超时。这时候我们再闭上眼睛，重新想象一遍，有序并且有转折点的一组数字....., 想明白了吗？max 值一定是 nums[i]+k 与 nums[n-1]-k 之中的一个，min 一定是 nums[0]+k 与 nums[i+1]-k 中的一个。想不明白的话再闭上眼，把坐标系上的排好序的点连起来形成一条线，然后从中间折断，左边的往上移，右边的往下移。

这里面还有一个细节问题，就是这个转折点 i 的取值范围的问题, 我一开始认为其取值范围应该是[0..n-2], 因为我觉得至少得有一个数字是+k 的, 要不然不可能做到减小 diff 作用。但是写完一提交，有一个 nums=[7, 8, 8], k = 5 的测试没有通过，而且平台给出的答案是 1，这时我才恍然大悟，有可能原数组已经做到 diff 最小了，这时候我们应该整体+k 或者整体-k。

---

代码(Rust):

```rust
impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        nums.sort();
        let mut ans = 10000;
        for i in 0..nums.len() {
            let min_left = nums[0] + k;
            let min_right = nums[i] + k;
            if i == nums.len() - 1 {
                ans = ans.min(min_right - min_left);
                break;
            }
            let add_left = nums[i + 1] - k;
            let add_right = nums[nums.len() - 1] - k;
            let min = min_left.min(min_right).min(add_left).min(add_right);
            let max = min_left.max(min_right).max(add_left).max(add_right);
            ans = ans.min(max - min);
        }
        ans
    }
}
```
