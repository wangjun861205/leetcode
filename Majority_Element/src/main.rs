struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut curr = nums[0];
        for v in nums.into_iter().skip(1) {
            if count == 0 {
                count = 1;
                curr = v;
            } else {
                if v == curr {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }
        curr
    }
}
fn main() {
    println!("{}", Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
