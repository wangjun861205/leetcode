struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_index = 0_usize;
        let mut right_index = height.len() - 1;
        let mut ans = 0;
        while left_index < right_index {
            let area =
                height[right_index].min(height[left_index]) * (right_index - left_index) as i32;
            if area > ans {
                ans = area;
            }
            if height[left_index] <= height[right_index] {
                left_index += 1;
            } else {
                right_index -= 1;
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
