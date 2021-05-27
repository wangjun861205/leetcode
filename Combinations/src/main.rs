struct Solution;

impl Solution {
    fn rc(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if k == 1 {
            return nums.into_iter().map(|v| vec![v]).collect();
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let mut remain = nums.clone();
            remain.drain(..=i);
            let l = Solution::rc(remain, k - 1);
            for mut v in l {
                v.insert(0, n);
                ans.push(v);
            }
        }
        ans
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let nums: Vec<i32> = (1..=n).into_iter().collect();
        Solution::rc(nums, k)
    }
}
fn main() {
    println!("Hello, world!");
}
