struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut queue = Vec::new();
        for n in arr {
            if queue.len() < k as usize {
                queue.push(n);
            } else {
                if (queue[0] - x).abs() > (n - x).abs() {
                    queue.remove(0);
                    queue.push(n);
                }
            }
        }
        queue
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
    );
}
