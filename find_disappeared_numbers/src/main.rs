struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut cl = vec![0; nums.len()];
        nums.into_iter().for_each(|v| {
            cl[(v - 1) as usize] += 1;
        });
        cl.into_iter()
            .enumerate()
            .filter(|(_, c)| *c == 0)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}
