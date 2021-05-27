struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let zero_pos: Vec<usize> = nums
            .iter()
            .enumerate()
            .filter(|(i, v)| *v == &0 && *i != nums.len() - 1)
            .map(|(i, _)| i)
            .collect();
        'outer: for p in zero_pos {
            for i in (0..p).rev() {
                if nums[i] as usize + i > p {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
