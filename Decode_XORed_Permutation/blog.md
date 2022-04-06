There is an integer array perm that is a permutation of the first n positive integers, where n is always odd.

It was encoded into another integer array encoded of length n - 1, such that encoded[i] = perm[i] XOR perm[i + 1]. For example, if perm = [1,3,2], then encoded = [2,1].

Given the encoded array, return the original array perm. It is guaranteed that the answer exists and is unique.

Example 1:

> Input: encoded = [3,1]  
> Output: [1,2,3]

Explanation: If perm = [1,2,3], then encoded = [1 XOR 2,2 XOR 3] = [3,1]

Example 2:

> Input: encoded = [6,5,4,6]  
> Output: [2,4,1,5,3]

Constraints:

- 3 <= n < 105
- n is odd.
- encoded.length == n - 1

---

答案看不懂系列，盯着人家给的答案看了半天，总算理解了人家的用意

整体来看，我们有了`encoded`, 所以我们只要找到`perm[0]`， 剩下的元素我们都可以很简单的算出来。

关于`perm[0]`， 题目给出的条件里`n`为奇数是关键

假设`x = perm[0] ^ perm[1] ^ perm[2]...perm[n-1]`, 已知`perm`为 1 到 n 的数组， 所以`x = 1 ^ 2 ^3...n`, 而`(perm[1] ^ perm[2]) ^ (perm[3] ^ perm[4])...(perm[n-2] ^ perm[n-1])`可以转换成`encoded[1] ^ encoded[3] ^ encoded[5] ... encoded[n-2]`, 所以最终可以推出`perm[0] = 1 ^ 2 ^ 3...n ^ encoded[1] ^ encoded[3] ^ encoded[5]... encoded[n-2]`

---

```rust
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; encoded.len() + 1];
        for i in 0..=ans.len() as i32 {
            ans[0] ^= i;
            if i < ans.len() as i32 && i % 2 == 1 {
                ans[0] ^= encoded[i as usize];
            }
        }
        for i in 1..ans.len() {
            ans[i] = ans[i - 1] ^ encoded[i - 1];
        }
        ans
    }
}
```
