struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![nums];
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            let mut remain = nums.clone();
            remain.remove(i);
            let l = Solution::permute(remain);
            for mut ll in l {
                ll.insert(0, nums[i]);
                ans.push(ll);
            }
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}
