struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut queue = Vec::new();
        let mut sum = 0;
        let mut sub_a: Option<(usize, usize)> = None;
        let mut sub_b: Option<(usize, usize)> = None;
        for (i, n) in arr.into_iter().enumerate() {
            queue.push(n);
            sum += n;
            while sum > target {
                sum -= queue.remove(0);
            }
            if sum == target {
                if let Some((start_a, end_a)) = sub_a {
                    if let Some((start_b, end_b)) = sub_b {
                        let length_a = end_a - start_a + 1;
                        let length_b = end_b - start_b + 1;
                        let start = i + 1 - queue.len();
                        let end = i;
                        let length = queue.len();

                        continue;
                    }
                    sub_b = Some((i + 1 - queue.len(), i));
                    continue;
                }
                sub_a = Some((i + 1 - queue.len(), i));
            }
        }
        if sub_a.is_none() || sub_b.is_none() {
            return -1;
        }
        let (start_a, end_a) = sub_a.unwrap();
        let (start_b, end_b) = sub_b.unwrap();
        (end_a - start_a + end_b - start_b + 2) as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_sum_of_lengths(vec![2, 1, 3, 3, 2, 3, 1], 6)
    );
}
