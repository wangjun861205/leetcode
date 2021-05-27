struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ord_idx: Vec<(usize, i32)> =
            nums.iter().enumerate().map(|(i, v)| (i, *v)).collect();
        ord_idx.sort_by_key(|v| v.1);
        if ord_idx.len() == 0 {
            return Vec::new();
        }
        let len = nums.len();
        let mut ans: Vec<i32> = Vec::new();
        loop {
            let (idx, min) = ord_idx.remove(0);
            if len - idx < k as usize {
                continue;
            }
            ans.push(min);
            if k == 1 {
                break;
            }
            let mut next = Solution::most_competitive(nums[idx + 1..].to_vec(), k - 1);
            if next.len() == 0 {
                continue;
            }
            ans.append(&mut next);
            break;
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::most_competitive(vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2], 3)
    );
}
