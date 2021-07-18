struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut ans = 0;
        nums.sort();
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] > nums[k] {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::triangle_number(vec![1]));
}
