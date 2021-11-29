struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target < nums[0] {
            return 0;
        }
        if target > *nums.last().unwrap() {
            return nums.len() as i32;
        }
        let mut l = 0_usize;
        let mut r = nums.len() - 1;
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] > target {
                r = m;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                return m as i32;
            }
        }
        l as i32
    }
}
fn main() {
    println!("Hello, world!");
}
