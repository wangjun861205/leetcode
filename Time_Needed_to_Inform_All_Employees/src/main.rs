struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(id: usize, subs: &HashMap<usize, Vec<usize>>, inform_time: &Vec<i32>) -> i32 {
        let mut ans = inform_time[id];
        if let Some(s) = subs.get(&id) {
            let mut max = 0;
            for sub in s {
                let next = Solution::rc(*sub, subs, inform_time);
                max = max.max(next);
            }
            ans += max;
        }
        ans
    }
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let subs: HashMap<usize, Vec<usize>> =
            manager
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut s, (i, v)| {
                    if v >= 0 {
                        s.entry(v as usize).or_insert(Vec::new()).push(i);
                    }
                    s
                });
        Solution::rc(head_id as usize, &subs, &inform_time)
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0])
    );
}
