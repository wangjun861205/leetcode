struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut changes: Vec<i32> = gas.iter().zip(cost.iter()).map(|(g, c)| *g - *c).collect();
        if changes.iter().sum::<i32>() < 0 {
            return -1;
        }
        let mut presum = 0;
        let mut ans = -1;
        for (i, c) in changes.into_iter().enumerate() {
            if c >= 0 {
                presum += c;
                if ans == -1 {
                    ans = i as i32;
                }
            } else {
                if presum + c >= 0 {
                    presum += c;
                } else {
                    presum = 0;
                    ans = -1;
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
}
