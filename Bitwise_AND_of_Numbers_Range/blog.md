Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.

Example 1:

> Input: left = 5, right = 7  
> Output: 4

Example 2:

> Input: left = 0, right = 0  
> Output: 0

Example 3:

> Input: left = 1, right = 2147483647  
> Output: 0

Constraints:

- 0 <= left <= right <= 231 - 1

---

都在代码里了

---

```rust
//  0 -> 00000 2 ** 0 - 1
//  1 -> 00001 2 ** 1 - 1
//  2 -> 00010
//  3 -> 00011 2 ** 2 - 1
//  4 -> 00100
//  5 -> 00101
//  6 -> 00110
//  7 -> 00111 2 ** 3 - 1
//  8 -> 01000
//  9 -> 01001
// 10 -> 01010
// 11 -> 01011
// 12 -> 01100
// 13 -> 01101
// 14 -> 01110
// 15 -> 01111 2 ** 4 - 1
// 16 -> 10000

// [2 ** n,  2 ** (n+1) - 1]   此范围内二进制第n-1位为1， 然后把n-1位置0， 递归新的区间

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == 0 {
            return 0;
        }
        if left == right {
            return left;
        }
        for n in 0..31 {
            if left >= 2_i32.pow(n) && right <= 2_i32.pow(n + 1) - 1 {
                return 1 << n | Solution::range_bitwise_and(left - (1 << n), right - (1 << n));
            }
        }
        return 0;
    }
}
```
