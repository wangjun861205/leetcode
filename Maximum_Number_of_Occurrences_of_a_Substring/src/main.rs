struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for n in min_size..=max_size {
            let mut max = 0;
            let mut left = 0_usize;
            let mut right = n as usize - 1;
            let mut cm: HashMap<char, i32> = chars[left..=right].into_iter().fold(HashMap::new(), |mut m, c| {
                *m.entry(*c).or_insert(0) += 1;
                m
            });
            let mut m = HashMap::new();
            while right < chars.len() {
                if cm.len() <= max_letters as usize {
                    let ss = chars[left..=right].into_iter().collect::<String>();
                    let e = m.entry(ss).or_insert(0);
                    *e += 1;
                    max = max.max(*e);
                }
                if right == chars.len() - 1 {
                    break;
                }
                if *cm.get(&chars[left]).unwrap() == 1 {
                    cm.remove(&chars[left]);
                } else {
                    *cm.get_mut(&chars[left]).unwrap() -= 1;
                }
                left += 1;
                right += 1;
                *cm.entry(chars[right]).or_insert(0) += 1;
            }
            count = count.max(max);
        }
        count
    }
}

fn main() {
    println!("{}", Solution::max_freq("abcde".to_owned(), 2, 3, 3));
}
