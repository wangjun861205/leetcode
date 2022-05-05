A certain bug's home is on the x-axis at position x. Help them get there from position 0.

The bug jumps according to the following rules:

It can jump exactly a positions forward (to the right).
It can jump exactly b positions backward (to the left).
It cannot jump backward twice in a row.
It cannot jump to any forbidden positions.
The bug may jump forward beyond its home, but it cannot jump to positions numbered with negative integers.

Given an array of integers forbidden, where forbidden[i] means that the bug cannot jump to the position forbidden[i], and integers a, b, and x, return the minimum number of jumps needed for the bug to reach its home. If there is no possible sequence of jumps that lands the bug on position x, return -1.

Example 1:

> Input: forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9  
> Output: 3

Explanation: 3 jumps forward (0 -> 3 -> 6 -> 9) will get the bug home.

Example 2:

> Input: forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11  
> Output: -1

Example 3:

> Input: forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7  
> Output: 2

Explanation: One jump forward (0 -> 16) then one jump backward (16 -> 7) will get the bug home.

Constraints:

- 1 <= forbidden.length <= 1000
- 1 <= a, b, forbidden[i] <= 2000
- 0 <= x <= 2000
- All the elements in forbidden are distinct.
- Position x is not forbidden.

---

盯着人家的答案看了一上午， 基本看明白了

- 能到达的点必定是 n \* gcd(a, b), 这个楼主说是根据 Bezout's Identity 得来的
- 关于检查上限， 假设出发点为 p0, 往前走一步为 p0 + a, 但是在[p0, p0+a]这个区间里还有若干个点是可以到达的[p0 + gcd(a, b), p0 + 2 * gcd(a, b), ..., p0 + a - gcd(a, b)], 这些点是从后面的点退一步 b 的距离到达的, 后面的那些点是[p0 + gcd(a, b) + b, p0 + 2 * gcd(a, b) + b, ... , p0 + a - gcd(a, b) + b], 所以我们要检查 p0 能否到达，只需要检测到 p0 + a - gcd(a, b) + b 这个点就可以了, 这个当时看答案看了好久
- 关于为什么要取 max(x, max(forbidden))来做 p0, 这个目前还没理解

---

```rust
use std::collections::HashSet;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        let c = a % b;
        Solution::gcd(b, c)
    }
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let gcd = Solution::gcd(a, b);
        if x % gcd != 0 {
            return -1;
        }
        let forbidden: HashSet<i32> = forbidden.into_iter().collect();
        let boundary = x.max(*forbidden.iter().max().unwrap()) + a + b;
        let mut stack = vec![(0, -1)];
        let mut visited: HashSet<(i32, i32)> = vec![(0, -1)].into_iter().collect();
        let mut steps = 0;
        loop {
            let mut new_stack = Vec::new();
            for (v, dir) in stack {
                if v == x {
                    return steps;
                }
                if v + a <= boundary
                    && !forbidden.contains(&(v + a))
                    && !visited.contains(&(v + a, 1))
                {
                    visited.insert((v + a, 1));
                    new_stack.push((v + a, 1));
                }
                if dir != -1
                    && v - b > 0
                    && !forbidden.contains(&(v - b))
                    && !visited.contains(&(v - b, -1))
                {
                    visited.insert((v - b, -1));
                    new_stack.push((v - b, -1));
                }
            }
            if new_stack.is_empty() {
                return -1;
            }
            steps += 1;
            stack = new_stack;
        }
    }
}

```
