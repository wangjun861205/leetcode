struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let diff = nums.iter().fold(0, |s, v| s ^ *v);
        let mut bit = 0;
        for i in 0..32 {
            if diff & (1 << i) > 0 {
                bit = 1 << i;
                break;
            }
        }
        let mut ans = vec![0, 0];
        nums.into_iter().for_each(|v| {
            if v & bit == 0 {
                ans[0] ^= v;
            } else {
                ans[1] ^= v;
            }
        });
        ans
    }
}
fn main() {
    println!("{:?}", Solution::single_number(vec![1, 0]));
}
