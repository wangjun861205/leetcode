The Tribonacci sequence Tn is defined as follows:

T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.

Given n, return the value of Tn.

Example 1:

> Input: n = 4  
> Output: 4  
> Explanation:
> T_3 = 0 + 1 + 1 = 2
> T_4 = 1 + 1 + 2 = 4

Example 2:

Input: n = 25  
Output: 1389537

Constraints:

- 0 <= n <= 37
- The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.

---

fibonacci 变成了 tribonacci， 变化不大，无非是从前 2 个数的和变成了前 3 个数的和。但是要注意一点如果用  
tib(n) = tib(n-1) + tib(n-2) + tib(n-3)的方式当 n 比较大的时候会超时， 所以我们用反向的方式

---

代码实现(Rust):

```rust
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut l = vec![0, 1, 1];
        match n {
            0 => return 0,
            1 => return 1,
            2 => return 1,
            _ => {
                for i in 3..=n as usize {
                    l.push(l[i - 3] + l[i - 2] + l[i - 1]);
                }
                return *l.last().unwrap();
            }
        }
    }
}
```
