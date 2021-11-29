struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut curr = 0;
        for n in nums {
            curr += n;
            min = min.min(curr);
        }
        if min < 1 {
            min.abs() + 1
        } else {
            min
        }
    }
}
fn main() {
    println!("{}", Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
}
