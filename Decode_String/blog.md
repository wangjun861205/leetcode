Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.

Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].

Example 1:

> Input: s = "3[a]2[bc]"  
> Output: "aaabcbc"

Example 2:

> Input: s = "3[a2[c]]"  
> Output: "accaccacc"

Example 3:

> Input: s = "2[abc]3[cd]ef"  
> Output: "abcabccdcdcdef"

Example 4:

> Input: s = "abc3[cd]xyz"  
> Output: "abccdcdcdxyz"

Constraints:

- 1 <= s.length <= 30
- s consists of lowercase English letters, digits, and square brackets '[]'.
- s is guaranteed to be a valid input.
- All the integers in s are in the range [1, 300].

---

每次读取一个字符, 根据字符的类型不同，有以下几种状态:

- 数字: push 到 times_str, 留作后面转换成 times 来用
- '[': 递归调用，拿到下层的返回字符串，然后解析 times_str 得到 times, repeat 返回的字符串 times 次，然后推入 ans
- ']': 直接返回 ans
- 其他: push 到 ans 中

---

代码实现(Rust):

```rust
impl Solution {
    fn decode(s: &mut String) -> String {
        let mut ans = String::new();
        let mut times_str = String::new();
        while !s.is_empty() {
            let c = s.remove(0);
            if c.is_numeric() {
                times_str.push(c);
            } else if c == '[' {
                let next = Solution::decode(s);
                let times = times_str.parse::<usize>().unwrap();
                ans.push_str(&next.repeat(times));
                times_str.clear();
            } else if c == ']' {
                return ans;
            } else {
                ans.push(c);
            }
        }
        ans
    }
    pub fn decode_string(mut s: String) -> String {
        Solution::decode(&mut s)
    }
}
```
