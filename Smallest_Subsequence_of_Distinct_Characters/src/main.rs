struct Solution;

impl Solution {
    fn rc(
        chars: &Vec<char>,
        index: usize,
        mask: i32,
        bitsets: &Vec<i32>,
        included: &mut Vec<bool>,
    ) -> Vec<char> {
        if index >= chars.len() {
            return Vec::new();
        }
        let mut char = chars[index];
        if included[char as usize - 97] {
            return Solution::rc(chars, index + 1, mask, bitsets, included);
        }
        let mut idx = index;
        for (i, c) in chars.iter().enumerate().skip(index + 1) {
            if c < &char && bitsets[i] | mask == bitsets[0] && !included[*c as usize - 97] {
                char = *c;
                idx = i;
            }
        }
        included[char as usize - 97] = true;
        let mut l = Solution::rc(
            chars,
            idx + 1,
            mask | (1 << (char as u8 - 97)),
            bitsets,
            included,
        );
        l.insert(0, char);
        l
    }

    pub fn smallest_subsequence(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut bitsets: Vec<i32> = s
            .chars()
            .rev()
            .scan(0, |m, c| {
                *m = ((1 << (c as u8 - 97)) | *m) as i32;
                Some(*m)
            })
            .collect();
        bitsets.reverse();
        let mut included = vec![false; 26];
        Solution::rc(&chars, 0, 0, &bitsets, &mut included)
            .into_iter()
            .collect()
    }
}

fn main() {
    println!("{}", Solution::smallest_subsequence("leetcode".to_owned()));
}
