struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let length = nums.len();
        let mut ans = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            if ans.is_empty() {
                ans.push(num);
                continue;
            }
            while let Some(last) = ans.pop() {
                if k as usize - ans.len() > length - i {
                    ans.push(last);
                    break;
                }
                if last <= num {
                    ans.push(last);
                    break;
                }
            }
            if ans.len() < k as usize {
                ans.push(num);
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)
    );
}
