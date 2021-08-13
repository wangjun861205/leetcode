You are given an integer num. You can swap two digits at most once to get the maximum valued number.

Return the maximum valued number you can get.

Example 1:

> Input: num = 2736  
> Output: 7236

Explanation: Swap the number 2 and the number 7.

Example 2:

> Input: num = 9973  
> Output: 9973

Explanation: No swap.

Constraints:

- 0 <= num <= 108

---

这种题应该说是挺常见的， 假设交换的两个 index 分别是 i, j, 如果想要得到最大的结果，i 一定要尽量小，而 nums[j]一定是 nums[i]之后的最大值。这两个条件前者的优先级要高于后者。所以最后交换的时候我们只需要按正序遍历即可，有符合条件 2 的情况，立马交换，然后跳出整个遍历即可。代码里面我把 nums[i]之后的最大值先计算出来了，因为这样可以大大降低时间复杂度，但是因为这题的数字长度有限，所以在交换的时候再遍历查找最大值也没什么问题，不过如果这题稍微变个形， 输入参数不是整数而是字符串的话，这就是必须的了，一般那种输入长度会非常变态。

---

```rust
impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut nums = Vec::new();
        // 拆分成单个的数字放入数组
        while num > 0 {
            let remain = num % 10;
            nums.insert(0, remain);
            num = num / 10;
        }
        // 计算ith之后的最大值组成数组
        let (mut max, mut max_idx) = (*nums.last().unwrap(), nums.len() - 1);
        let mut maxs = vec![(0, 0); nums.len()];
        for i in (0..nums.len()).rev() {
            if nums[i] > max {
                max = nums[i];
                max_idx = i;
            }
            maxs[i] = (max, max_idx);
        }
        'outer: for i in 0..nums.len() {
            let (max, max_idx) = maxs[i];
            if nums[i] < max {
                let temp = nums[i];
                nums[i] = max;
                nums[max_idx] = temp;
                break 'outer;
            }
        }
        nums.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap()
    }
}
```
