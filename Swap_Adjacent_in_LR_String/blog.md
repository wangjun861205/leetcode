In a string composed of 'L', 'R', and 'X' characters, like "RXXLRXRXL", a move consists of either replacing one occurrence of "XL" with "LX", or replacing one occurrence of "RX" with "XR". Given the starting string start and the ending string end, return True if and only if there exists a sequence of moves to transform one string to the other.

Example 1:

> Input: start = "RXXLRXRXL", end = "XRLXXRRLX"  
> Output: true

Explanation:
We can transform start to end following these steps:  
RXXLRXRXL ->  
XRXLRXRXL ->  
XRLXRXRXL ->  
XRLXXRRXL ->  
XRLXXRRLX

Example 2:

> Input: start = "X", end = "L"  
> Output: false

Example 3:

> Input: start = "LLR", end = "RRL"  
> Output: false

Example 4:

> Input: start = "XL", end = "LX"  
> Output: true

Example 5:

> Input: start = "XLLR", end = "LXLX"  
> Output: false

Constraints:

- 1 <= start.length <= 104
- start.length == end.length
- Both start and end will only consist of characters in 'L', 'R', and 'X'.

---

貌似大家都不太待见这题，顶跟踩的差不多一半一半。先说两种变换的方式， "RX" -> "XR", "XL" -> "LX", 其实说白了就是 R 的右边是 X 的时候 R 可以向右移动，L 的左边是 X 的时候 L 可以向左移动。然后我们把问题拆分成四种情况:

1. start[i] == 'X', end[i] == 'R'
2. start[i] == 'X', end[i] == 'L'
3. start[i] == 'R', end[i] == 'X'
4. start[i] == 'L', end[i] == 'X'

其他 start[i] == end[i]的情况，我们不用考虑，因为不需要进行变换

对应的四种变换方式,或者说是查找方式:

1. 从 start[i]到 start[0]\(注意是反向\)查找'R', 如果遇到'X'则 continue， 如果遇到'L'则直接返回 false。找到后与 start[i]进行互换。
2. 从 start[i]到 start[end]查找'L', 如果遇到'X'则 continue， 如果遇到'R'则直接返回 false。找到后与 start[i]进行互换。
3. 从 start[i]到 start[end]查找'X', 如果遇到'R'则 continue, 如果遇到'L'则直接返回 false。找到后与 start[i]进行互换。
4. 从 start[i]到 start[0]\(注意是反向\)查找'X', 如果遇到'L'则 continue, 如果遇到'R'则直接返回 false。找到后与 start[i]进行互换。

---

代码实现(Rust):

```rust
impl Solution {
    fn find_l(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in i..chars.len() {
            match chars[j] {
                'L' => {
                    return Some(j);
                }
                'X' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_r(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in (0..=i).rev() {
            match chars[j] {
                'R' => return Some(j),
                'X' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_rx(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in i..chars.len() {
            match chars[j] {
                'X' => return Some(j),
                'R' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_lx(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in (0..=i).rev() {
            match chars[j] {
                'X' => return Some(j),
                'L' => continue,
                _ => return None,
            }
        }
        None
    }

    pub fn can_transform(start: String, end: String) -> bool {
        let mut start_chars: Vec<char> = start.chars().collect();
        let end_chars: Vec<char> = end.chars().collect();
        for i in 0..start_chars.len() {
            let s = start_chars[i];
            let e = end_chars[i];
            if s == e {
                continue;
            }
            match e {
                'X' => match s {
                    'R' => {
                        if let Some(j) = Solution::find_rx(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        if let Some(j) = Solution::find_lx(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                },
                'R' => match s {
                    'X' => {
                        if let Some(j) = Solution::find_r(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                },
                _ => match s {
                    'X' => {
                        if let Some(j) = Solution::find_l(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                },
            }
        }
        true
    }
}
```
