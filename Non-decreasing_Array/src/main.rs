struct Solution;

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut index = 0;
        for (i, w) in nums.windows(2).enumerate() {
            if w[0] > w[1] {
                count += 1;
                index = i;
            }
            if count > 1 {
                return false;
            }
        }
        if count == 0 {
            return true;
        }
        if index == 0 || index == nums.len() - 2 {
            return true;
        } else {
            return nums[index - 1] <= nums[index + 1] || nums[index] <= nums[index + 2];
        }
    }
}
fn main() {
    println!("{}", Solution::check_possibility(vec![-1, 4, 2, 3]));
}
