struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut gt_count = 0;
        let mut lt_count = 0;
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut ans = 0;
        while right < hours.len() {
            if hours[right] > 8 {
                gt_count += 1;
                right += 1;
                ans = ans.max(right - left);
            } else {
                if gt_count - lt_count > 1 {
                    lt_count += 1;
                    right += 1;
                    ans = ans.max(right - left);
                } else {
                    left = right;
                    right += 1;
                }
            }
        }
        ans as i32
    }
}

fn main() {
    println!("{}", Solution::longest_wpi(vec![6, 6, 9, 9]));
}
