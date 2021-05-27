struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut presum: Vec<i32> = vec![0; 121];
        for &a in ages.iter() {
            presum[a as usize] += 1;
        }
        for i in 1..presum.len() {
            presum[i] += presum[i - 1];
        }
        let mut ans = 0;
        for a in ages {
            if a <= 14 {
                continue;
            }
            ans += presum[a as usize] - presum[(a / 2 + 7) as usize] - 1;
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_friend_requests(vec![20, 30, 100, 110, 120])
    );
}
