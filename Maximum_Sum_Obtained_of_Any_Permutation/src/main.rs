struct Solution;

impl Solution {
    pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        nums.sort();
        nums.reverse();
        let mut counts: Vec<i32> = vec![0; nums.len()];
        for r in requests {
            counts[r[0] as usize] += 1;
            if r[1] != nums.len() as i32 - 1 {
                counts[r[1] as usize + 1] -= 1;
            }
        }
        for i in 1..counts.len() {
            counts[i] += counts[i - 1];
        }
        counts = counts.into_iter().filter(|v| v > &0).collect();
        counts.sort();
        counts.reverse();
        (nums
            .into_iter()
            .zip(counts)
            .map(|(n, c)| n as i64 * c as i64)
            .sum::<i64>()
            % (10_i64.pow(9) + 7)) as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_sum_range_query(
            vec![1, 2, 3, 4, 5, 10],
            vec![vec![0, 2], vec![1, 3], vec![1, 1]]
        )
    );
}
