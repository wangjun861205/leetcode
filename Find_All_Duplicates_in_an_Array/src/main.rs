struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            let idx = nums[i].abs() as usize - 1;
            if nums[idx] < 0 {
                ans.push(idx as i32 + 1);
            }
            nums[idx] = -nums[idx]
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}
