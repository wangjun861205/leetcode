struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 1;
        }
        let mut ans = 0;
        let mut index: usize = 1;
        let mut diff = 0;
        while index < nums.len() {
            match nums[index] - nums[index - 1] {
                d if d < 0 => {
                    if diff < 0 {
                        index += 1;
                    } else if diff == 0 {
                        index += 1;
                        diff = d;
                    } else {
                        ans += 1;
                        index += 1;
                        diff = d;
                    }
                }
                d if d == 0 => {
                    index += 1;
                }
                d => {
                    if diff < 0 {
                        ans += 1;
                        index += 1;
                        diff = d;
                    } else if diff == 0 {
                        index += 1;
                        diff = d;
                    } else {
                        index += 1;
                    }
                }
            }
        }
        if ans == 0 {
            if diff == 0 {
                1
            } else {
                2
            }
        } else {
            ans + 2
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    );
    println!("{}", Solution::wiggle_max_length(vec![0, 0, 0]));
    println!(
        "{}",
        Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8])
    );
    println!("{}", Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
}
