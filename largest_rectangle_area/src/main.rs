struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let mut index = 0_usize;
        while index < heights.len() {
            if stack.len() == 0 || heights[*stack.last().unwrap()] < heights[index] {
                stack.push(index);
                index += 1;
            } else {
                let top_of_stack = stack.pop().unwrap();
                let area = heights[top_of_stack]
                    * (if let Some(last) = stack.last() {
                        index - *last - 1
                    } else {
                        index
                    }) as i32;
                max_area = max_area.max(area);
            }
        }
        while stack.len() > 0 {
            let top_of_stack = stack.pop().unwrap();
            let area = heights[top_of_stack]
                * (if let Some(last) = stack.last() {
                    index - *last - 1
                } else {
                    index
                }) as i32;
            max_area = max_area.max(area);
        }
        max_area
    }
}

fn main() {
    println!(
        "{}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
}
