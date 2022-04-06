struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    fn rc(mut heap: BinaryHeap<(char, i32)>, repeat_limit: i32, ans: &mut String) {
        if let Some((c, mut v)) = heap.pop() {
            if let Some(last_char) = ans.chars().last() {
                if c == last_char {
                    if let Some((nc, mut nv)) = heap.pop() {
                        nv -= 1;
                        ans.push(nc);
                        if nv > 0 {
                            heap.push((nc, nv));
                        }
                        if v > 0 {
                            heap.push((c, v));
                        }
                        Solution::rc(heap, repeat_limit, ans);
                    } else {
                        return;
                    }
                } else {
                    let n = repeat_limit.min(v);
                    v -= n;
                    ans.push_str(&c.to_string().repeat(n as usize));
                    if v > 0 {
                        heap.push((c, v));
                    }
                    Solution::rc(heap, repeat_limit, ans);
                }
            } else {
                let n = repeat_limit.min(v);
                v -= n;
                ans.push_str(&c.to_string().repeat(n as usize));
                if v > 0 {
                    heap.push((c, v));
                }
                Solution::rc(heap, repeat_limit, ans);
            }
        }
    }
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut counts = vec![0; 26];
        for c in s.chars() {
            counts[c as usize - 97] += 1;
        }
        let heap: BinaryHeap<(char, i32)> = counts
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v > &0)
            .map(|(i, v)| ((i as u8 + 97) as char, v))
            .collect();
        let mut ans = String::new();
        Solution::rc(heap, repeat_limit, &mut ans);
        ans
    }
}

fn main() {
    println!("{}", Solution::repeat_limited_string("aababab".into(), 2));
}
