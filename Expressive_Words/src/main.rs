struct Solution;

impl Solution {
    fn group(s: String) -> Vec<(char, i32)> {
        let group: Vec<(char, i32)> = s.chars().fold(Vec::new(), |mut l, v| {
            if let Some((c, cnt)) = l.last_mut() {
                if *c == v {
                    *cnt += 1;
                    return l;
                }
            }
            l.push((v, 1));
            l
        });
        group
    }

    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let target_group = Solution::group(s);
        let mut count = 0;
        'outer: for w in words {
            let group = Solution::group(w);
            if group.len() != target_group.len() {
                continue;
            }
            for (g, tg) in group.iter().zip(target_group.iter()) {
                if g.0 != tg.0 {
                    continue 'outer;
                }
                if tg.1 < g.1 {
                    continue 'outer;
                }
                if tg.1 == g.1 {
                    continue;
                }
                if tg.1 < 3 {
                    continue 'outer;
                }
            }
            count += 1;
        }
        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::expressive_words(
            "dddiiiinnssssssoooo".to_owned(),
            vec![
                "dinnssoo",
                "ddinso",
                "ddiinnso",
                "ddiinnssoo",
                "ddiinso",
                "dinsoo",
                "ddiinsso",
                "dinssoo",
                "dinso"
            ]
            .into_iter()
            .map(str::to_owned)
            .collect()
        )
    );
}
