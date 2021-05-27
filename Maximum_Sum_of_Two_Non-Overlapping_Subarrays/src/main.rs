struct Solution;

use std::cmp::Reverse;
use std::collections::HashMap;

impl Solution {
    fn rc(
        l_sum: &Vec<((usize, usize), i32)>,
        m_sum: &Vec<((usize, usize), i32)>,
        l_idx: usize,
        m_idx: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if l_idx == l_sum.len() || m_idx == m_sum.len() {
            return 0;
        }
        let l = l_sum[l_idx];
        let m = m_sum[m_idx];
        let (l_start, l_end) = (l.0 .0, l.0 .1);
        let (m_start, m_end) = (m.0 .0, m.0 .1);
        if l_end < m_start || m_end < l_start {
            return l.1 + m.1;
        } else {
            let next_l = if let Some(c) = cache.get(&(l_idx + 1, m_idx)) {
                *c
            } else {
                Solution::rc(l_sum, m_sum, l_idx + 1, m_idx, cache)
            };
            let next_m = if let Some(c) = cache.get(&(l_idx, m_idx + 1)) {
                *c
            } else {
                Solution::rc(l_sum, m_sum, l_idx, m_idx + 1, cache)
            };
            return next_l.max(next_m);
        }
    }
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        let mut l_sum: Vec<((usize, usize), i32)> = a
            .windows(l as usize)
            .enumerate()
            .map(|(i, v)| ((i, i + l as usize - 1), v.into_iter().sum()))
            .collect();
        let mut m_sum: Vec<((usize, usize), i32)> = a
            .windows(m as usize)
            .enumerate()
            .map(|(i, v)| ((i, i + m as usize - 1), v.into_iter().sum()))
            .collect();
        l_sum.sort_by_key(|v| Reverse(v.1));
        m_sum.sort_by_key(|v| Reverse(v.1));
        let mut cache = HashMap::new();
        Solution::rc(&l_sum, &m_sum, 0, 0, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_sum_two_no_overlap(
            vec![
                55, 85, 51, 92, 84, 21, 84, 92, 0, 72, 93, 51, 44, 26, 22, 53, 31, 57, 60, 8, 69,
                13, 27, 86, 14, 92, 47, 62, 11, 99, 54, 5, 16, 51, 27, 85, 37, 73, 16, 51, 36, 29,
                84, 80, 46, 97, 84, 16, 20, 13
            ],
            40,
            10
        )
    );
}
