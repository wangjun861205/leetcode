Given an array of integers arr of even length, return true if and only if it is possible to reorder it such that arr[2 * i + 1] = 2 _ arr[2 _ i] for every 0 <= i < len(arr) / 2.

Example 1:

> Input: arr = [3,1,3,6]  
> Output: false

Example 2:

> Input: arr = [2,1,2,6]  
> Output: false

Example 3:

> Input: arr = [4,-2,2,-4]  
> Output: true

Explanation:  
We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].

Example 4:

> Input: arr = [1,2,4,16,8,4]  
> Output: false

Constraints:

- 0 <= arr.length <= 3 \* 104
- arr.length is even.
- -105 <= arr[i] <= 105

---

1. 根据绝对值排序, 因为有负数
2. 遍历查找每个元素看其倍数是不是存在, 如果存在则将元素和其倍数挪出集合

这里比较绕的地方是为什么我们可以只查元素的倍数而不查 n/2？因为我们已经做过排序，如果这个数的 1/2 存在的的话，那应该已经在前面的步骤中查过了。举个例子[1, 2, 6, 12], 我们之所以不查 6 的 1/2 也就是 3，那是因为如果 3 存在在这个数组里，那在检查 3 的时候，6 会被找到，而 3 和 6 都会在这个数组内移除，后面的步骤就不会再查找 6 的倍数了。

---

代码实现(Rust):

```rust
use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        arr.sort_by_key(|v| v.abs());
        let mut map: HashMap<i32, i32> = arr.iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
        for v in arr {
            if map.get(&v).unwrap() == &0 {
                continue;
            }
            *map.get_mut(&v).unwrap() -= 1;

            if !map.contains_key(&(v * 2)) || map.get(&(v * 2)).unwrap() == &0 {
                return false;
            }
            *map.get_mut(&(v * 2)).unwrap() -= 1;
        }
        true
    }
}
```
