You are given two strings a and b that consist of lowercase letters. In one operation, you can change any character in a or b to any lowercase letter.

Your goal is to satisfy one of the following three conditions:

Every letter in a is strictly less than every letter in b in the alphabet.
Every letter in b is strictly less than every letter in a in the alphabet.
Both a and b consist of only one distinct letter.
Return the minimum number of operations needed to achieve your goal.

Example 1:

> Input: a = "aba", b = "caa"  
> Output: 2

Explanation: Consider the best way to make each condition true:

1. Change b to "ccc" in 2 operations, then every letter in a is less than every letter in b.
2. Change a to "bbb" and b to "aaa" in 3 operations, then every letter in b is less than every letter in a.
3. Change a to "aaa" and b to "aaa" in 2 operations, then a and b consist of one distinct letter.
   The best way was done in 2 operations (either condition 1 or condition 3).

Example 2:

> Input: a = "dabadd", b = "cda"  
> Output: 3

Explanation: The best way is to make condition 1 true by changing b to "eee".

Constraints:

- 1 <= a.length, b.length <= 105
- a and b consist only of lowercase letters.

---

先分别计算 a 和 b 中各字母出现的频次， 然后把按每个字母作为分隔的情况下， 需要挪动的字母数量都计算出来取最小值， 当然还有如果都挪到同一个字母， 那需要选择出现最多频次的字母

---

```rust
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let counts_a = a.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let counts_b = b.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let length = (a.len() + b.len()) as i32;
        let mut a_left_b_right = i32::MAX;
        let mut a_right_b_left = i32::MAX;
        let mut only_one = i32::MAX;
        for i in 1..26 {
            a_left_b_right = a_left_b_right.min(
                counts_a[..i].into_iter().sum::<i32>() + counts_b[i..].into_iter().sum::<i32>(),
            );
            a_right_b_left = a_right_b_left.min(
                counts_b[..i].into_iter().sum::<i32>() + counts_a[i..].into_iter().sum::<i32>(),
            );
        }
        for i in 0..26 {
            only_one = only_one.min(length - counts_a[i] - counts_b[i]);
        }
        a_left_b_right.min(a_right_b_left).min(only_one)
    }
}
```
