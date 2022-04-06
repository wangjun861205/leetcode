A wonderful string is a string where at most one letter appears an odd number of times.

For example, "ccjjc" and "abab" are wonderful, but "ab" is not.
Given a string word that consists of the first ten lowercase English letters ('a' through 'j'), return the number of wonderful non-empty substrings in word. If the same substring appears multiple times in word, then count each occurrence separately.

A substring is a contiguous sequence of characters in a string.

Example 1:

> Input: word = "aba"  
> Output: 4

Explanation: The four wonderful substrings are underlined below:

- "aba" -> "a"
- "aba" -> "b"
- "aba" -> "a"
- "aba" -> "aba"

Example 2:

> Input: word = "aabb"  
> Output: 9

Explanation: The nine wonderful substrings are underlined below:

- "aabb" -> "a"
- "aabb" -> "aa"
- "aabb" -> "aab"
- "aabb" -> "aabb"
- "aabb" -> "a"
- "aabb" -> "abb"
- "aabb" -> "b"
- "aabb" -> "bb"
- "aabb" -> "b"

Example 3:

> Input: word = "he"
> Output: 2

Explanation: The two wonderful substrings are underlined below:

- "he" -> "h"
- "he" -> "e"

Constraints:

- 1 <= word.length <= 105
- word consists of lowercase English letters from 'a' to 'j'.

---

1. 用 bitmask 的形式来表现一个字符串中每个字符的奇偶状态
2. 根据条件， 我们找到与当前 bitmask 相同和差一位的所有**_之前的_**字符串的数量

注意代码中`counts[0] = 1`这一句， 应为 bitmask 为 0 的时候所有字符出现的次数均为偶数， 所以是符合最多出现一个字符为奇数的条件的，所以需要初始化为 1， 其实就是字符串本身不用减去任何前缀字符串就可以满足条件。

---

```rust
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut counts = vec![0i64; 1024];
        counts[0] = 1;
        let mut mask = 0;
        let mut ans = 0;
        for c in word.chars() {
            mask ^= 1 << (c as usize - 97);
            ans += counts[mask];
            for i in 0..10 {
                ans += counts[mask ^ (1 << i)];
            }
            counts[mask] += 1;
        }
        ans
    }
}
```
