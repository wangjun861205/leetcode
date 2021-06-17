struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| (v[0], v[1]));
        let mut stack: Vec<Vec<i32>> = Vec::new();
        let mut count = 0;
        for itv in intervals {
            if !stack.is_empty() {
                let last = stack.last().unwrap().clone();
                if itv[0] < last[1] {
                    if itv[1] >= last[1] {
                        count += 1;
                        continue;
                    } else {
                        stack.pop();
                        stack.push(itv);
                        count += 1;
                        continue;
                    }
                }
                stack.push(itv);
            } else {
                stack.push(itv);
            }
        }
        count
    }
}
fn main() {
    println!(
        "{}",
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
    );
}
