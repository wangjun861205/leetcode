struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        let mut left_less = vec![0; heights.len()];
        let mut right_less = vec![0; heights.len()];
        left_less[0] = -1;
        *right_less.last_mut().unwrap() = heights.len() as i32;
        for i in 1..heights.len() as i32 {
            let mut prev = i - 1;
            while prev >= 0 && heights[prev as usize] >= heights[i as usize] {
                prev = left_less[prev as usize];
            }
            left_less[i as usize] = prev;
        }
        for i in (0..heights.len() as i32 - 1).rev() {
            let mut prev = i + 1;
            while prev < heights.len() as i32 && heights[prev as usize] >= heights[i as usize] {
                prev = right_less[prev as usize];
            }
            right_less[i as usize] = prev;
        }
        let mut ans = 0;
        for i in 0..heights.len() {
            ans = ans.max((right_less[i] - left_less[i] - 1) * heights[i]);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::largest_rectangle_area(vec![9, 0]));
}
