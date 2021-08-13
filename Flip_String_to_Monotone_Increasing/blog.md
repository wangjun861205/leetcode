A binary string is monotone increasing if it consists of some number of 0's (possibly none), followed by some number of 1's (also possibly none).

You are given a binary string s. You can flip s[i] changing it from 0 to 1 or from 1 to 0.

Return the minimum number of flips to make s monotone increasing.

Example 1:

> Input: s = "00110"  
> Output: 1

Explanation:  
We flip the last digit to get 00111.

Example 2:

> Input: s = "010110"  
> Output: 2  
> Explanation:  
> We flip to get 011111, or alternatively 000111.

Example 3:

> Input: s = "00011000"
> Output: 2

Explanation:  
We flip to get 00000000.

Constraints:

- 1 <= s.length <= 105
- s[i] is either '0' or '1'.

---

奥运会正在举行，本来我对运动除了 motogp，其他的都没有什么兴趣。但是最近做题突然有点感想，就是这类 Dynamic Programming 的题目，解起来就像是跨栏。你的步子的落点直接决定你的成绩，落点不对，你跑再快也得撞栏上。落点对了你自然而然的会跑的很轻松。所以怎么把这类问题合理的切割成一个个可重复且有递归关系的类型一致的小问题是解这类问题的关键。

回到这道题上来， 那上面的例子来说:

> Input: s = "00011000"

首先我们能想到的是正序遍历这个字符串, 0 的情况我们可以忽略不管，因为只要连续的 0，我们就可以认为之前遍历过的那些字符串都是符合单调性的。关键是 1 的情况，遇到 1 的情况我们有两种选择，一是把后面的所有 0 都变成 1，二是这个字符本身变成 0，然后我们只需要保证后面的字符串变成单调字符串就可以了。前者很简单，我们做一个 0 的倒序 presum，然后直接查就可以得到所需要变的步数。后者就只能靠 dp 来解决了。然后把这两种情况的步数对比一下取最小值就可以了。

就上面的输入来说, 第一个 1 的 index 是 3, 如果后面剩余的字符串都变成 1 的话， 也就是"1000"变成"1111"， 一共 3 个 0，所以需要 3 次 flap。如果我们把 s[3]变成 0, 那整个字符串变成"00001000", 我们只要让 s[4..]也就是"1000"变成单调字符串就行了。

假设第一个 1 的 index 为 k, zero_count(i)为从 ith 往后的 0 数量之和
dp[0..n] = min(zero_count(k), dp[k+1..n] + 1)

---

```rust

impl Solution {
    fn dp(chars: &Vec<char>, i: usize, zero_count: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        let mut first_one_index = None;
        for j in i..chars.len() {
            if chars[j] == '1' {
                first_one_index = Some(j);
                break;
            }
        }
        if let Some(idx) = first_one_index {
            if idx == chars.len() - 1 {
                return 0;
            }
            let non_flap = zero_count[idx];
            let flap = if cache[idx + 1] != -1 {
                cache[idx + 1]
            } else {
                Solution::dp(chars, idx + 1, zero_count, cache)
            };
            let ans = non_flap.min(flap + 1);
            cache[i] = ans;
            return ans;
        }
        0
    }

    pub fn min_flips_mono_incr(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut zero_count: Vec<i32> = chars
            .iter()
            .rev()
            .scan(0, |c, v| {
                if v == &'0' {
                    *c += 1;
                }
                Some(*c)
            })
            .collect();
        zero_count.reverse();
        let mut cache = vec![-1; chars.len()];
        Solution::dp(&chars, 0, &zero_count, &mut cache)
    }
}

```
