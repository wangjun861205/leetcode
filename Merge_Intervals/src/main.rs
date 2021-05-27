struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|v| v[0]);
        let mut stack: Vec<Vec<i32>> = Vec::new();
        for v in intervals {
            if stack.is_empty() {
                stack.push(v);
                continue;
            }
            let prev = stack.last_mut().unwrap();
            if prev[1] < v[0] {
                stack.push(v);
                continue;
            }
            if prev[1] >= v[1] {
                continue;
            }
            prev[1] = v[1];
        }
        stack
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
}
