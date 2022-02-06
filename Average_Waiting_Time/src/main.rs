struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let length = customers.len();
        let mut time = 0_f64;
        let mut wait_time = 0_f64;
        for c in customers {
            if (c[0] as f64) < time {
                wait_time += time - c[0] as f64;
            } else {
                time = c[0] as f64;
            }
            wait_time += c[1] as f64;
            time += c[1] as f64;
        }
        wait_time / length as f64
    }
}

fn main() {
    println!(
        "{}",
        Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]])
    );
}
