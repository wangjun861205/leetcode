You are given two identical eggs and you have access to a building with n floors labeled from 1 to n.

You know that there exists a floor f where 0 <= f <= n such that any egg dropped at a floor higher than f will break, and any egg dropped at or below floor f will not break.

In each move, you may take an unbroken egg and drop it from any floor x (where 1 <= x <= n). If the egg breaks, you can no longer use it. However, if the egg does not break, you may reuse it in future moves.

Return the minimum number of moves that you need to determine with certainty what the value of f is.

Example 1:

> Input: n = 2  
> Output: 2

Explanation: We can drop the first egg from floor 1 and the second egg from floor 2.
If the first egg breaks, we know that f = 0.
If the second egg breaks but the first egg didn't, we know that f = 1.
Otherwise, if both eggs survive, we know that f = 2.

Example 2:

Input: n = 100  
Output: 14

Explanation: One optimal strategy is:

- Drop the 1st egg at floor 9. If it breaks, we know f is between 0 and 8. Drop the 2nd egg starting from floor 1 and going up one at a time to find f within 8 more drops. Total drops is 1 + 8 = 9.
- If the 1st egg does not break, drop the 1st egg again at floor 22. If it breaks, we know f is between 9 and 21. Drop the 2nd egg starting from floor 10 and going up one at a time to find f within 12 more drops. Total drops is 2 + 12 = 14.
- If the 1st egg does not break again, follow a similar process dropping the 1st egg from floors 34, 45, 55, 64, 72, 79, 85, 90, 94, 97, 99, and 100.
  Regardless of the outcome, it takes at most 14 drops to determine f.

Constraints:

- 1 <= n <= 1000

---

DP 的方法， 从 1 到 n 的每一层(i), 我们扔一枚鸡蛋， 如果这枚鸡蛋破了我们只剩下一枚完好的鸡蛋， 那我们只能从第一层开始逐层测试， 这样就需要抛 n-1 次， 加上刚才破的那一次， 也就是说如果在某一层鸡蛋破了， 我们整体需要 n 次测试才能测出来。如果鸡蛋没破， 那 1..i 层就都不需要测试了， 我们只需要去测试剩下的 n-i 层。为了考虑最坏的情况， 我们要取破跟不破的结果之中最大的。

---

```rust
impl Solution {
    fn dp(n: i32, cache: &mut Vec<i32>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut ans = i32::MAX;
        for i in 1..=n {
            let broken = i;
            let survive = if cache[(n - i) as usize] > 0 {
                cache[(n - i) as usize] + 1
            } else {
                Solution::dp(n - i, cache) + 1
            };
            ans = ans.min(broken.max(survive));
        }
        cache[n as usize] = ans;
        ans
    }

    pub fn two_egg_drop(n: i32) -> i32 {
        Solution::dp(n, &mut vec![0; n as usize + 1])
    }
}
```
