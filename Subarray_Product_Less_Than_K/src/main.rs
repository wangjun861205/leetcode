struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut prod = 1;
        let mut count = 0;
        while right < nums.len() {
            prod *= nums[right];
            while left <= right && prod >= k {
                prod /= nums[left];
                left += 1;
            }
            count += right - left + 1;
            right += 1;
        }
        count as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_subarray_product_less_than_k(
            vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
            19
        )
    );
}
