struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut matrix = vec![vec![0; nums.len()]; goal as usize + 2];
        if nums[0] == 0 {
            matrix[0][0] = 1;
        }
        for i in 1..nums.len() {
            if nums[i] == 0 {
                matrix[0][i] = matrix[0][i - 1] + 1;
            }
        }
        if nums[0] == 1 {
            matrix[1][0] = 1;
        }
        for i in 1..nums.len() {
            if nums[i] == 1 {
                matrix[1][i] = matrix[0][i - 1] + 1;
            } else {
                matrix[1][i] = matrix[1][i - 1];
            }
        }
        for i in 2..=goal as usize {
            for j in 1..nums.len() {
                if nums[j] == 0 {
                    matrix[i][j] = matrix[i][j - 1]
                } else {
                    matrix[i][j] = matrix[i - 1][j - 1]
                }
            }
        }
        println!("{:?}", matrix);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += matrix[goal as usize][i]
        }
        sum
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2)
    );
}
