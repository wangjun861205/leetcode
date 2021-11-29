struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let zero_count = nums.iter().filter(|v| v == &&0).count();
        if zero_count > 1 {
            return vec![0; nums.len()];
        }
        let prod = nums.iter().fold(1, |p, v| {
            if v == &0 {
                return p;
            }
            p * *v
        });
        if zero_count == 1 {
            return nums
                .into_iter()
                .map(|v| if v == 0 { prod } else { 0 })
                .collect();
        }
        nums.into_iter().map(|v| prod / v).collect()
    }
}
fn main() {
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, 3, -3]));
}
