Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k.

Example 1:

> Input: s = "aaabb", k = 3  
> Output: 3

Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.

Example 2:

> Input: s = "ababbc", k = 2  
> Output: 5

Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.

Constraints:

- 1 <= s.length <= 104
- s consists of only lowercase English letters.
- 1 <= k <= 105

---

猛地一看觉得挺简单， 真正解起来费了好长时间。整体思路是遍历整个字符串计出每个字母的出现频次， 把频次`<k`的的字母筛选出来， 包含这些字母的字符串一定是不合格的字符串， 如果不存在频次`<k`的字母, 则当前整个字符串都是合法的，其长度就是最大合法子字符串的长度。如果存在这些不合格字母， 我们则把原字符串根据这些不合格字母进行分割， 形成若干个子字符串， 这里要注意， 虽然这些子字符串不包含不合格字母， 但是他们本身不一定就是合法的， 所以我们需要递归调用再对这些子字符串进行处理。

例:

> s = "aaabbcb"  
> k = 3

第一步找到不合格字符是 c, 依据 c 进行分割得到两个子字符串:

> s1 = "aaabb"  
> s2 = "b"

我们先处理 s1, s1 中的 b 此时变成了不合格字母， 因为 k=3 而这里面的 b 只有 2 个， 这样我们在根据 b 来对当前字符串进行分割:

> s11 = "aaa"

这时 s11 里面不包含不合格字母了， 那整个字符串的长度就是当前字符的最长合法子字符串的长度, 即是 3

我们再回头看 s2, b 在这里显然也是不合格字母， 而 s2 又只包含 1 个 b, 所以分割之后的字符串即是空字符串， 子字符串的长度为 0

最后我们对 s1 和 s2 的返回长度进行对比， 取最大值， 这样最终的答案就是 3

---

```rust
use std::collections::HashSet;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let counts = s.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let invalid_chars: HashSet<char> = counts
            .into_iter()
            .enumerate()
            .filter(|(_, count)| count != &0 && count < &k)
            .map(|(i, _)| (i as u8 + 97) as char)
            .collect();
        if invalid_chars.is_empty() {
            return s.len() as i32;
        }
        let mut ans = 0;
        let mut ss = String::new();
        for c in s.chars() {
            if !invalid_chars.contains(&c) {
                ss.push(c);
                continue;
            }
            if !ss.is_empty() {
                ans = ans.max(Solution::longest_substring(ss, k));
            }
            ss = String::new();
        }
        if !ss.is_empty() {
            ans = ans.max(Solution::longest_substring(ss, k));
        }
        ans
    }
}
```
