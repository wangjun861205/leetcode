You have d dice and each die has f faces numbered 1, 2, ..., f. You are given three integers d, f, and target.

Return the number of possible ways (out of fd total ways) modulo 109 + 7 to roll the dice so the sum of the face-up numbers equals target.

Example 1:

> Input: d = 1, f = 6, target = 3  
> Output: 1

Explanation:  
You throw one die with 6 faces. There is only one way to get a sum of 3.

Example 2:

> Input: d = 2, f = 6, target = 7  
> Output: 6

Explanation:
You throw two dice, each with 5 faces. There are 6 ways to get a sum of 7:
1+6, 2+5, 3+4, 4+3, 5+2, 6+1.

Example 3:

> Input: d = 2, f = 5, target = 10  
> Output: 1

Explanation:  
You throw two dice, each with 5 faces. There is only one way to get a sum of 10: 5+5.

Example 4:

> Input: d = 1, f = 2, target = 3  
> Output: 0

Explanation:  
You throw one die with 2 faces. There is no way to get a sum of 3.

Example 5:

> Input: d = 30, f = 30, target = 500  
> Output: 222616187

Explanation:  
The answer must be returned modulo 10^9 + 7.

Constraints:

- 1 <= d, f <= 30
- 1 <= target <= 1000

---

去年 10 月 29 号做这个题， 多次尝试都失败了， 今天重新做， 20 分钟解决战斗， 感觉不错， 至少证明了自己在这一年里还是有进步的。
典型的 dp 问题， 不做赘述， 看代码

---

```rust

use std::collections::HashMap;

impl Solution {
    fn dp(d: i32, f: i32, target: i32, cache: &mut HashMap<(i32, i32, i32), i64>) -> i64 {
        if d == 1 {
            if target > f {
                return 0;
            } else {
                return 1;
            }
        }
        const M: i64 = 1000000007;
        let mut sum = 0_i64;
        for v in 1..=f {
            if target - v > 0 {
                let next = if let Some(c) = cache.get(&(d - 1, f, target - v)) {
                    *c
                } else {
                    Solution::dp(d - 1, f, target - v, cache)
                };
                sum += next % M;
                sum %= M;
            }
        }
        sum %= M;
        cache.insert((d, f, target), sum);
        sum
    }
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        Solution::dp(d, f, target, &mut HashMap::new()) as i32
    }
}
```
