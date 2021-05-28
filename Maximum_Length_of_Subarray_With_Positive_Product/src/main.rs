struct Solution;

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut first_negative = -1;
        let mut zero_index = -1;
        let mut is_negative = false;
        let mut ans = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if n == 0 {
                first_negative = -1;
                is_negative = false;
                zero_index = i as i32;
            } else if n > 0 {
                if is_negative {
                    ans = ans.max(i as i32 - first_negative);
                } else {
                    ans = ans.max(i as i32 - zero_index);
                }
            } else {
                if is_negative {
                    is_negative = false;
                    ans = ans.max(i as i32 - zero_index);
                } else {
                    if first_negative == -1 {
                        first_negative = i as i32;
                        is_negative = true;
                    } else {
                        is_negative = true;
                        ans = ans.max(i as i32 - first_negative);
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::get_max_len(vec![1, 2, 3, 5, -6, 4, 0, 10]));
}
