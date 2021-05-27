struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let l: Vec<i32> = words
            .iter()
            .map(|s| {
                let mut bits = 0;
                for c in s.chars() {
                    bits |= 1 << (c as u8 - 'a' as u8);
                }
                bits
            })
            .collect();
        let mut ans = 0;
        for (i, n) in l.iter().enumerate() {
            for (j, nn) in l.iter().enumerate().skip(i + 1) {
                if n & nn == 0 {
                    let prod = words[i].len() * words[j].len();
                    if prod > ans {
                        ans = prod
                    }
                }
            }
        }
        ans as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_product(
            vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        )
    );
}
