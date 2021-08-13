Given a string s, partition s such that every substring of the partition is a palindrome.

Return the minimum cuts needed for a palindrome partitioning of s.

Example 1:

> Input: s = "aab"  
> Output: 1

Explanation:  
The palindrome partitioning ["aa","b"] could be produced using 1 cut.

Example 2:

> Input: s = "a"  
> Output: 0

Example 3:

> Input: s = "ab"  
> Output: 1

Constraints:

- 1 <= s.length <= 2000
- s consists of lower-case English letters only.

---

既然让咱们 cut 那咱们就 cut 呗，从左向右找下刀的点，只要保证切下来左半部分是回文就可以下刀，又半部分递归调用 cut, 继续这个过程，整个过程没有切错了这一说，因为最坏的情况是你把每个字母都单独切出来，这也算是回文，只是刀数是最大的。这里面要考虑一个边界情况就是一刀不切人家就是回文的情况，这种情况直接返回 0 就可以了，因为这已经是最好的情况了。

---

代码(Rust):

```rust
use std::collections::HashMap;

impl Solution {
    fn is_palindrome(chars: &[char]) -> bool {
        let mut i = 0 as i32;
        let mut j = chars.len() as i32 - 1;
        while i < j {
            if chars[i as usize] != chars[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
    pub fn cut(chars: &Vec<char>, cache: &mut HashMap<Vec<char>, i32>) -> i32 {
        if chars.is_empty() {
            return 0;
        }
        if Solution::is_palindrome(chars) {
            return 0;
        }
        let mut ans = chars.len() as i32 - 1;
        for i in 0..chars.len() {
            if Solution::is_palindrome(&chars[..=i]) {
                let remain = if let Some(c) = cache.get(&chars[i + 1..]) {
                    *c
                } else {
                    Solution::cut(&chars[i + 1..].to_vec(), cache)
                };
                ans = ans.min(remain);
            }
        }
        cache.insert(chars[..].to_vec(), ans + 1);
        ans + 1
    }
    pub fn min_cut(s: String) -> i32 {
        let mut cache = HashMap::new();
        Solution::cut(&s.chars().collect(), &mut cache)
    }
}
```
