Given an array of strings arr. String s is a concatenation of a sub-sequence of arr which have unique characters.

Return the maximum possible length of s.

Example 1:

> Input: arr = ["un","iq","ue"]  
> Output: 4  
> Explanation: All possible concatenations are "","un","iq","ue","uniq" and "ique".
> Maximum length is 4.

Example 2:

> Input: arr = ["cha","r","act","ers"]  
> Output: 6  
> Explanation: Possible solutions are "chaers" and "acters".

Example 3:

> Input: arr = ["abcdefghijklmnopqrstuvwxyz"]  
> Output: 26

Constraints:

- 1 <= arr.length <= 16
- 1 <= arr[i].length <= 26
- arr[i] contains only lower case English letters.

---

将每个 string 转化为 26 个字母的 bitset， 这里注意一点， 一个单词内部也有可能出现重复的字母，所以转换的时候要做检查。转换完成用 dp 的方式就可以解出来了。简单来说就是从前向后按顺序来，分别计算拿取这个单词和略过这个单词的单词长度，这里要考虑当前的单词的 bitset 和前面已经混合好的 bitset，有冲突和没冲突的情况的计算方式不一样。

---

代码实现(Rust):

```rust
use std::collections::HashMap;

impl Solution {
    fn dp(mut bitsets: Vec<i32>, mut bitset: i32, cache: &mut HashMap<(i32, usize), i32>) -> i32 {
        if bitsets.is_empty() {
            let mut count = 0;
            for _ in 0..32 {
                count += (bitset & 1);
                bitset >>= 1;
            }
            return count;
        }
        let set = bitsets.remove(0);
        let pick = if bitset & set > 0 {
            if let Some(c) = cache.get(&(set, bitsets.len())) {
                *c
            } else {
                Solution::dp(bitsets.clone(), set, cache)
            }
        } else {
            if let Some(c) = cache.get(&(bitset | set, bitsets.len())) {
                *c
            } else {
                Solution::dp(bitsets.clone(), bitset | set, cache)
            }
        };
        let pass = if let Some(c) = cache.get(&(bitset, bitsets.len())) {
            *c
        } else {
            Solution::dp(bitsets.clone(), bitset, cache)
        };
        let ans = pick.max(pass);
        cache.insert((bitset, bitsets.len()), ans);
        ans
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let bitsets: Vec<i32> = arr
            .into_iter()
            .map(|s| {
                let mut set = 0;
                for c in s.chars() {
                    if set & 1 << (c as usize - 97) > 0 {
                        return -1;
                    }
                    set |= 1 << (c as usize - 97);
                }
                set
            })
            .filter(|v| v > &0)
            .collect();
        let mut cache = HashMap::new();
        Solution::dp(bitsets, 0, &mut cache)
    }
}
```
