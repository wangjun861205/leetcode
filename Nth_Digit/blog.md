Given an integer n, return the nth digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].

Example 1:

> Input: n = 3  
> Output: 3

Example 2:

> Input: n = 11  
> Output: 0

Explanation: The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.

Constraints:

- 1 <= n <= 231 - 1

---

注意每次生成新的范围时, `curr=next+1`, 如果不+1 那就会导致范围有交叠

---

```rust
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        if n < 10 {
            return n;
        }
        let mut i = 0;
        let mut curr = 0_i64;
        let mut next = 9_i64;
        while next < n as i64 {
            i += 1;
            curr = next + 1;
            next += 9 * 10_i64.pow(i as u32) * (i + 1);
        }
        let start = 10_i64.pow(i as u32);
        let offset = (n as i64 - curr) / (i + 1);
        let index = (n as i64 - curr) % (i + 1);
        (start + offset)
            .to_string()
            .chars()
            .nth(index as usize)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap()
    }
}
```
