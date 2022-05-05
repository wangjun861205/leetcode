Given an array of characters chars, compress it using the following algorithm:

Begin with an empty string s. For each group of consecutive repeating characters in chars:

If the group's length is 1, append the character to s.
Otherwise, append the character followed by the group's length.
The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.

After you are done modifying the input array, return the new length of the array.

You must write an algorithm that uses only constant extra space.

Example 1:

> Input: chars = ["a","a","b","b","c","c","c"]  
> Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]

Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".

Example 2:

> Input: chars = ["a"]  
> Output: Return 1, and the first character of the input array should be: ["a"]

Explanation: The only group is "a", which remains uncompressed since it's a single character.

Example 3:

> Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]  
> Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].

Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".

Constraints:

- 1 <= chars.length <= 2000
- chars[i] is a lowercase English letter, uppercase English letter, digit, or symbol.

---

因为要求 constant space, 所以我们只能在原有的数组上进行写入， 好消息是这种压缩编码方式， 所生成的字符串的长度一定是小于等于原字符串长度的， 这样我们设定两个指针，一个作为读取的指针，表示我们读取的位置， 另一个作为写入的指针，表示我们写入的位置, 写入指针是永远无法超过读取指针的。剩下的事就是读取、编码、写入，循环往复

---

```rust
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut read_index = 0usize;
        let mut write_index = 0usize;
        let mut prev_char = None;
        let mut count = 0;
        while read_index < chars.len() {
            let curr = chars[read_index];
            if let Some(prev) = prev_char {
                if curr == prev {
                    count += 1;
                    read_index += 1;
                    continue;
                }
                if count > 1 {
                    for c in format!("{}{}", prev, count).chars() {
                        chars[write_index] = c;
                        write_index += 1;
                    }
                } else {
                    chars[write_index] = prev;
                    write_index += 1;
                }
                read_index += 1;
                count = 1;
                prev_char = Some(curr);
                continue;
            }
            prev_char = Some(curr);
            read_index = 1;
            count = 1;
        }
        if count > 1 {
            for c in format!("{}{}", prev_char.as_ref().unwrap(), count).chars() {
                chars[write_index] = c;
                write_index += 1;
            }
        } else {
            chars[write_index] = *prev_char.as_ref().unwrap();
            write_index += count;
        }
        chars.truncate(write_index);
        chars.len() as i32
    }
}
```
