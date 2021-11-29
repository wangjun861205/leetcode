You are given a string s consisting only of characters 'a' and 'b'​​​​.

You can delete any number of characters in s to make s balanced. s is balanced if there is no pair of indices (i,j) such that i < j and s[i] = 'b' and s[j]= 'a'.

Return the minimum number of deletions needed to make s balanced.

Example 1:

> Input: s = "aababbab"  
> Output: 2  
> Explanation: You can either:  
> Delete the characters at 0-indexed positions 2 and 6 ("aababbab" -> "aaabbb"), or
> Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").

Example 2:

> Input: s = "bbaaaaabb"  
> Output: 2  
> Explanation: The only solution is to delete the first two characters.

Constraints:

- 1 <= s.length <= 105
- s[i] is 'a' or 'b'​​.

---

1. 先计算出每个字符后面的 a 的数量
2. 遍历字符， a 不用考虑， 遇到 b 有两种选择， 一是删除当前的 b， 继续向后走。 二是删除当前 b 后面的所有 a 然后结束。
3. 需要考虑把 a 全删掉和把 b 全删掉的情况

---

```rust
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a_count = 0;
        let mut b_count = 0;
        let mut remain_a: Vec<i32> = s
            .chars()
            .rev()
            .scan(0, |l, v| {
                if v == 'a' {
                    a_count += 1;
                    *l += 1;
                    return Some(*l);
                }
                b_count += 1;
                Some(*l)
            })
            .collect();
        remain_a.reverse();
        let mut ans = i32::MAX;
        let mut removed_b = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'b' {
                ans = ans.min(removed_b + remain_a[i]);
                removed_b += 1;
            }
        }
        if ans == i32::MAX {
            return 0;
        }
        ans.min(a_count).min(b_count)
    }
}

```
