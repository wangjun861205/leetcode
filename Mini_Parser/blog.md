Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return the deserialized NestedInteger.

Each element is either an integer or a list whose elements may also be integers or other lists.

Example 1:

> Input: s = "324"  
> Output: 324

Explanation:  
You should return a NestedInteger object which contains a single integer 324.

Example 2:

> Input: s = "[123,[456,[789]]]"  
> Output: [123,[456,[789]]]

Explanation:  
Return a NestedInteger object containing a nested list with 2 elements:

1. An integer containing value 123.
2. A nested list containing two elements:
   i. An integer containing value 456.
   ii. A nested list with one element:
   a. An integer containing value 789

Constraints:

- 1 <= s.length <= 5 \* 104
- s consists of digits, square brackets "[]", negative sign '-', and commas ','.
- s is the serialization of valid NestedInteger.

---

状态机，有点差别的地方是， 如果在[]内返回，我们直接返回 List 就好， 但是如果不能确定是在[]内，我们要判断当前解析的元素数量，如果只有一个元素则返回 Int, 如果是多个则返回 List。具体的大家看代码吧

---

代码实现(Rust):

```rust
impl Solution {
    fn deser(chars: &mut Vec<char>) -> NestedInteger {
        let mut vals: Vec<NestedInteger> = Vec::new();
        let mut val_str = String::new();
        while !chars.is_empty() {
            let c = chars.remove(0);
            match c {
                '[' => {
                    let next = Solution::deser(chars);
                    vals.push(next);
                }
                ']' => {
                    if !val_str.is_empty() {
                        vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
                        val_str.clear();
                    }
                    return NestedInteger::List(vals);
                }
                ',' => {
                    if !val_str.is_empty() {
                        vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
                        val_str.clear();
                    }
                }
                _ => {
                    val_str.push(c);
                }
            }
        }
        if !val_str.is_empty() {
            vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
            val_str.clear();
        }
        if vals.len() == 1 {
            vals.pop().unwrap()
        } else {
            NestedInteger::List(vals)
        }
    }
    pub fn deserialize(s: String) -> NestedInteger {
        Solution::deser(&mut s.chars().collect())
    }
}
```
