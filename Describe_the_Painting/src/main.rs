struct Solution;

impl Solution {
    pub fn split_painting(mut segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        segments.sort_by_key(|v| v[0]);
        let mut sets: Vec<Option<i64>> = vec![None; 100001];
        for s in segments {
            if let Some(c) = &mut sets[s[0] as usize] {
                *c += s[2] as i64;
            } else {
                sets[s[0] as usize] = Some(s[2] as i64);
            }
            if let Some(c) = &mut sets[s[1] as usize] {
                *c -= s[2] as i64;
            } else {
                sets[s[1] as usize] = Some(-s[2] as i64);
            }
        }
        let mut ans = Vec::new();
        let mut curr = 0;
        let mut prev = None;
        for (i, v) in sets.into_iter().enumerate() {
            if let Some(v) = v {
                if prev.is_none() {
                    curr = v;
                    prev = Some(i);
                } else {
                    if curr == 0 {
                        curr += v;
                        prev = Some(i);
                    } else {
                        let p = prev.unwrap();
                        ans.push(vec![p as i64, i as i64, curr]);
                        curr += v;
                        prev = Some(i);
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::split_painting(vec![
            vec![4, 16, 12],
            vec![9, 10, 15],
            vec![18, 19, 13],
            vec![3, 13, 20],
            vec![12, 16, 3],
            vec![2, 10, 10],
            vec![3, 11, 4],
            vec![13, 16, 6]
        ])
    );
}
