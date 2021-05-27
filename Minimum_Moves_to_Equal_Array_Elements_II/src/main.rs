struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut count = 0;
        while i < j {
            count += nums[j] - nums[i];
            i += 1;
            j -= 1;
        }
        count
    }
}
fn main() {
    println!("{}", Solution::min_moves2(vec![1, 0, 0, 8, 6]));
}
