struct Solution;

// 0, 1, 2, 3, 4, 5, 6
// 0, 4, 1, 5, 2, 6, 3

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        nums.sort();
        let mut ans = Vec::new();
        let mut first = nums[..=length / 2].to_vec();
        let mut second = nums[length / 2 + 1..].to_vec();
        while !first.is_empty() {
            ans.push(first.remove(0));
            if !second.is_empty() {
                ans.push(second.remove(0));
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::rearrange_array(vec![1, 2, 3, 4, 5]));
}
