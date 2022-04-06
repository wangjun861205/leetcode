struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| v[0]);
        let mut max = intervals[0][1];
        let mut min = intervals[0][0];
        let mut ans = intervals.len() as i32;
        for v in intervals.into_iter().skip(1) {
            if v[1] <= max {
                ans -= 1;
            } else {
                if v[0] == min {
                    ans -= 1;
                } else {
                    min = v[0];
                }
                max = v[1];
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]])
    );
}
