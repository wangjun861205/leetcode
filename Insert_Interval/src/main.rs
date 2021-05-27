struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack: Vec<Vec<i32>> = Vec::new();
        let mut inserted = false;
        for v in intervals {
            if v[0] < new_interval[0] {
                stack.push(v);
                continue;
            }
            if !inserted {
                if stack.is_empty() {
                    stack.push(new_interval.clone());
                } else {
                    let prev = stack.last_mut().unwrap();
                    if prev[1] < new_interval[0] {
                        stack.push(new_interval.clone());
                    } else if prev[1] < new_interval[1] {
                        prev[1] = new_interval[1];
                    }
                }
                inserted = true;
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
        if !inserted {
            if stack.is_empty() {
                stack.push(new_interval);
            } else {
                let prev = stack.last_mut().unwrap();
                if prev[1] < new_interval[0] {
                    stack.push(new_interval.clone());
                } else if prev[1] < new_interval[1] {
                    prev[1] = new_interval[1];
                }
            }
        }
        stack
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    );
}
