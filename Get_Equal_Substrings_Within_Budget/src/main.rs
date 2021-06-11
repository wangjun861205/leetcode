struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let diffs: Vec<i32> = s
            .chars()
            .zip(t.chars())
            .map(|(sc, tc)| (sc as i32 - tc as i32).abs())
            .collect();
        println!("{:?}", diffs);
        let mut ans = 0;
        let mut sum = 0;
        let mut left = 0_usize;
        let mut right = 0_usize;
        while right < diffs.len() {
            sum += diffs[right];
            while sum > max_cost {
                sum -= diffs[left];
                left += 1;
            }
            ans = ans.max(right - left + 1);
            right += 1;
        }
        ans as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3)
    );
}
