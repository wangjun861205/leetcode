struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counts: BinaryHeap<(i32, char)> = s
            .chars()
            .fold(vec![0; 26], |mut l, c| {
                l[c as usize - 97] += 1;
                l
            })
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, (i as u8 + 97) as char))
            .filter(|(v, _)| v > &0)
            .collect();
        let mut ans = String::new();
        while !counts.is_empty() {
            let mut stack = Vec::new();
            let mut ok = false;
            while let Some((count, c)) = counts.pop() {
                if let Some(last) = ans.chars().last() {
                    if c == last {
                        stack.push((count, c));
                        continue;
                    }
                }
                ok = true;
                ans.push(c);
                stack.push((count - 1, c));
                break;
            }
            if !ok {
                return "".into();
            }
            for (count, c) in stack {
                if count > 0 {
                    counts.push((count, c));
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::reorganize_string("aaab".into()));
}
