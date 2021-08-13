struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut prev: Option<char> = None;
        let mut count = 0_usize;
        let mut groups = vec![Vec::new(); 26];
        for (i, c) in text.chars().enumerate() {
            if let Some(p) = prev {
                if c == p {
                    count += 1;
                } else {
                    groups[p as usize - 97].push((i - count, i - 1));
                    prev = Some(c);
                    count = 1;
                }
            } else {
                prev = Some(c);
                count += 1;
            }
        }
        groups[prev.unwrap() as usize - 97].push((text.len() - count, text.len() - 1));
        let mut ans = groups
            .iter()
            .max_by_key(|&l| {
                if !l.is_empty() {
                    l[0].1 - l[0].0 + 1
                } else {
                    0
                }
            })
            .map(|l| l[0].1 - l[0].0 + 1)
            .unwrap();
        groups.into_iter().for_each(|l| {
            l.windows(2).for_each(|w| {
                if w[1].0 - w[0].1 == 2 {
                    if l.len() > 2 {
                        ans = ans.max((w[0].1 - w[0].0) + (w[1].1 - w[1].0) + 3);
                    } else {
                        ans = ans.max((w[0].1 - w[0].0) + (w[1].1 - w[1].0) + 2);
                    }
                } else {
                    ans = ans.max((w[0].1 - w[0].0 + 2).max(w[1].1 - w[1].0 + 2))
                }
            })
        });
        ans as i32
    }
}

fn main() {
    println!("{}", Solution::max_rep_opt1("aaabbaaa".to_owned()));
}
