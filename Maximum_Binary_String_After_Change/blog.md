You are given a binary string binary consisting of only 0's or 1's. You can apply each of the following operations any number of times:

- Operation 1: If the number contains the substring "00", you can replace it with "10".
  For example, "00010" -> "10010"
- Operation 2: If the number contains the substring "10", you can replace it with "01".
  For example, "00010" -> "00001"
  Return the maximum binary string you can obtain after any number of operations. Binary string x is greater than binary string y if x's decimal representation is greater than y's decimal representation.

Example 1:

> Input: binary = "000110"  
> Output: "111011"

Explanation: A valid transformation sequence can be:  
"000110" -> "000101"  
"000101" -> "100101"  
"100101" -> "110101"  
"110101" -> "110011"  
"110011" -> "111011"

Example 2:

> Input: binary = "01"  
> Output: "01"

Explanation:

"01" cannot be transformed any further.

Constraints:

- 1 <= binary.length <= 105
- binary consist of '0' and '1'.

---

看几个例子:

"00" -> "10"
"010" -> "001" -> "101"
"0110" -> "0101" -> "0011" -> "1011"
"01110" -> "01101" -> "01011" -> "00111" -> "10111"
"101" -> "101"

如果我们单看开始和最终结果，我们可以观察到一个规律，在经过一系列变换之后，我们实际能做的有效的变换其实如下:

1. 字符串中如果只有 1 个'0'， 我们没法做有效变换
2. 字符串中如果有 2 个'0', 分别在 s[i], s[j], 且 i < j, 那我们可以做 s[i] = '1', s[j] = '1', s[i+1] = '0'

字符串中如果有多个'0', 那可以重复步骤 2, 因为进行变换之后，实际上是处于 s[i]的'0'移动到了 s[i+1]的位置上, 而 s[j]处的'0'转换成了'1'， 所以实际过程是个 s[i]那个'0'不断后移的过程，我们只需要设置一个变量记录前面的 0 的位置就可以了

---

代码实现(Rust):

```rust
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut chars: Vec<char> = binary.chars().collect();
        let mut prev_zero_index: Option<usize> = None;
        for i in 0..chars.len() {
            if chars[i] == '0' {
                if let Some(p) = prev_zero_index {
                    chars[p] = '1';
                    chars[i] = '1';
                    chars[p + 1] = '0';
                    prev_zero_index = Some(p + 1);
                } else {
                    prev_zero_index = Some(i);
                }
            }
        }
        chars.into_iter().collect()
    }
}
```
