struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut ans = 1;
        for i in 1..nums.len() {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                    ans = ans.max(dp[i]);
                }
            }
        }
        ans
    }
}

// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut sub = Vec::new();
//         sub.push(nums[0]);
//         for n in nums[1..].into_iter() {
//             if n > sub.last().unwrap() {
//                 sub.push(*n);
//             } else {
//                 let i = sub.iter().position(|v| v >= n).unwrap();
//                 sub[i] = *n;
//             }
//         }
//         sub.len() as i32
//     }
// }
fn main() {
    println!("Hello, world!");
}
