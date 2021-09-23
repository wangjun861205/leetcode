struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = 0;
        for n in nums {
            if n == 0 {
                ans = ans.max(count);
                count = 0;
            } else {
                count += 1;
            }
        }
        ans = ans.max(count);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
