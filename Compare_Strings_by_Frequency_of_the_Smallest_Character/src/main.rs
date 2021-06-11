struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut frq_slots: Vec<i32> = vec![0; 10];
        let flsc = |word: String| {
            let mut char_counts: Vec<i32> = vec![0; 26];
            for c in word.chars() {
                char_counts[(c as u8 - 97) as usize] += 1;
            }
            char_counts.into_iter().find(|v| v > &0).unwrap()
        };
        for w in words {
            let frq = flsc(w);
            frq_slots[(frq - 1) as usize] += 1;
        }
        let mut presum = vec![0; 10];
        for i in 0..10 {
            for j in 0..i {
                presum[j] += frq_slots[i];
            }
        }
        queries
            .into_iter()
            .map(|s| {
                let frq = flsc(s);
                presum[frq as usize - 1]
            })
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::num_smaller_by_frequency(
            vec!["bbb".to_owned(), "cc".to_owned()],
            vec![
                "a".to_owned(),
                "aa".to_owned(),
                "aaa".to_owned(),
                "aaaa".to_owned()
            ]
        )
    );
}
