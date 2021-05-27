struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_idx: Option<usize> = None;
        nums.into_iter()
            .enumerate()
            .filter(|(_, v)| *v == 1)
            .all(|(i, _)| {
                if let Some(idx) = prev_idx {
                    let ok = i - idx > k as usize;
                    prev_idx = Some(i);
                    return ok;
                } else {
                    prev_idx = Some(i);
                    return true;
                }
            })
    }
}
fn main() {
    println!(
        "{}",
        Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
    )
}
