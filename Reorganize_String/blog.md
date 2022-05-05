Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.

Return any possible rearrangement of s or return "" if not possible.

Example 1:

> Input: s = "aab"  
> Output: "aba"

Example 2:

> Input: s = "aaab"  
> Output: ""

Constraints:

- 1 <= s.length <= 500
- s consists of lowercase English letters.

---

这题一开始用 DP 的方法解， 怎么解怎么超时， 后来没办法看提示， `Alternate placing the most common letters.`, 想想， 貌似是这个道理。改变策略最终解决。具体方法是把字符串中每个字母的出现频次放到一个 BinaryHeap 中， 每次 pop 一下， 取出一个字母， 把该字母 push 到要返回的字符串中， 然后再把频次-1，然后再把该字母和频次放回 Heap 中， 过程中记得检查 pop 出的字母是不是与返回字符串的最后一个字母相同， 如果相同则继续对 heap 进行 pop 操作, 还有在放回字母和频次之前先检查一下频次是不是大于 0， 如果等于 0 那就不需要放回了。

---

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counts: BinaryHeap<(i32, char)> = s
            .chars()
            .fold(vec![0; 26], |mut l, c| {
                l[c as usize - 97] += 1;
                l
            })
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, (i as u8 + 97) as char))
            .filter(|(v, _)| v > &0)
            .collect();
        let mut ans = String::new();
        while !counts.is_empty() {
            let mut stack = Vec::new();
            let mut ok = false;
            while let Some((count, c)) = counts.pop() {
                if let Some(last) = ans.chars().last() {
                    if c == last {
                        stack.push((count, c));
                        continue;
                    }
                }
                ok = true;
                ans.push(c);
                stack.push((count - 1, c));
                break;
            }
            if !ok {
                return "".into();
            }
            for (count, c) in stack {
                if count > 0 {
                    counts.push((count, c));
                }
            }
        }
        ans
    }
}
```
