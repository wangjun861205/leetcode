struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let mut delta = vec![0; 2 * limit as usize + 2];
        let mut l = 0_usize;
        let mut r = nums.len() - 1;
        while l < r {
            let (a, b) = (nums[l], nums[r]);
            delta[2] += 2;
            delta[a.min(b) as usize + 1] -= 1;
            delta[(a + b) as usize] -= 1;
            delta[(a + b + 1) as usize] += 1;
            delta[(a.max(b) + limit + 1) as usize] += 1;
            l += 1;
            r -= 1;
        }
        let mut curr = 0;
        let mut ans = i32::MAX;
        for d in delta[2..].into_iter() {
            curr += *d;
            ans = ans.min(curr);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_moves(vec![1, 2, 2, 1], 2));
}
