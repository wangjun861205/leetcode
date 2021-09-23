You are given a binary string s. You are allowed to perform two types of operations on the string in any sequence:

Type-1: Remove the character at the start of the string s and append it to the end of the string.
Type-2: Pick any character in s and flip its value, i.e., if its value is '0' it becomes '1' and vice-versa.
Return the minimum number of type-2 operations you need to perform such that s becomes alternating.

The string is called alternating if no two adjacent characters are equal.

For example, the strings "010" and "1010" are alternating, while the string "0100" is not.

Example 1:

> Input: s = "111000"  
> Output: 2  
> Explanation: Use the first operation two times to make s = "100011".
> Then, use the second operation on the third and sixth elements to make s = "101010".

Example 2:

> Input: s = "010"  
> Output: 0  
> Explanation: The string is already alternating.

Example 3:

> Input: s = "1110"  
> Output: 1  
> Explanation: Use the second operation on the second element to make s = "1010".

Constraints:

- 1 <= s.length <= 105
- s[i] is either '0' or '1'.

---

开始想了很久，后来看了提示才发现自己真的是傻。无论中间过程怎么变化，最终只会有两种结果，一种是奇数位为 0，偶数位为 1，另一种是奇数位为 1，偶数位为 0。我们需要统计的无非就是这两种情况下我们需要做的 type2 操作的数量。稍微有点复杂的是这个题目可以 shift，不过我们只需要统计完成， 然后根据 shift 之后的情况修改计数就行了， 不用每次都统计。

---

代码实现(Rust):

```rust
impl Solution {
    fn shift(s: &mut String, even_zero: &mut i32, odd_one: &mut i32, even_one: &mut i32, odd_zero: &mut i32) {
        let c = s.remove(0);
        if c == '0' {
            *even_zero -= 1;
        } else {
            *even_one -= 1;
        }
        let mut temp = *even_zero;
        *even_zero = *odd_zero;
        *odd_zero = temp;
        temp = *even_one;
        *even_one = *odd_one;
        *odd_one = temp;
        if c == '0' {
            s.push(c);
            if s.len() % 2 == 0 {
                *odd_zero += 1;
            } else {
                *even_zero += 1;
            }
        } else {
            s.push(c);
            if s.len() % 2 == 0 {
                *odd_one += 1;
            } else {
                *even_one += 1;
            }
        }
    }
    pub fn min_flips(mut s: String) -> i32 {
        let length = s.len();
        let mut odd_zero = 0;
        let mut odd_one = 0;
        let mut even_zero = 0;
        let mut even_one = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                if i % 2 == 0 {
                    even_zero += 1;
                } else {
                    odd_zero += 1;
                }
            } else {
                if i % 2 == 0 {
                    even_one += 1;
                } else {
                    odd_one += 1;
                }
            }
        }
        let mut ans = i32::MAX;
        for _ in 0..length {
            ans = ans.min((odd_zero + even_one).min(odd_one + even_zero));
            Solution::shift(&mut s, &mut even_zero, &mut odd_one, &mut even_one, &mut odd_zero)
        }
        ans
    }
}
```
