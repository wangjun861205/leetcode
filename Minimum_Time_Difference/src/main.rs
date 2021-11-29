struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mins = |s: &str| -> i32 {
            let l: Vec<String> = s.split(":").map(str::to_string).collect();
            return l[0].parse::<i32>().unwrap() * 60 + l[1].parse::<i32>().unwrap();
        };
        let mut heap = BinaryHeap::new();
        for t in time_points {
            heap.push(mins(&t))
        }
        let l = heap.into_sorted_vec();
        let mut ans = i32::MAX;
        for w in l.windows(2) {
            ans = ans.min(w[1] - w[0]);
        }
        ans = ans.min(l.first().unwrap() + 24 * 60 - l.last().unwrap());
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_min_difference(vec![
            "00:00".to_owned(),
            "23:59".to_owned(),
            "00:00".to_owned()
        ])
    );
}
