struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut masks: Vec<i32> = s
            .chars()
            .rev()
            .scan(0, |s, c| {
                *s |= 1 << (c as usize - 97) as usize;
                Some(*s)
            })
            .collect();
        masks.reverse();
        let mut mask = masks[0];
        let mut ans: Vec<char> = Vec::new();
        let mut position = 0;
        while position < chars.len() {
            if mask & (1 << (chars[position] as usize - 97)) == 0 {
                position += 1;
                continue;
            }
            let mut picked = chars[position];
            for (i, c) in chars.iter().enumerate().skip(position + 1) {
                if mask & (1 << (*c as usize - 97)) == 0 {
                    continue;
                }
                if c < &picked && masks[i] & mask == mask {
                    picked = *c;
                    position = i;
                }
            }
            mask ^= 1 << (picked as usize - 97);
            ans.push(picked);
            position += 1;
        }
        ans.into_iter().collect()
    }
}
fn main() {
    println!(
        "{}",
        Solution::remove_duplicate_letters("mitnlruhznjfyzmtmfnstsxwktxlboxutbic".to_owned())
    );
}
