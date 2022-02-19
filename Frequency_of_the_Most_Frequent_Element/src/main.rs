struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut stack = vec![nums[0]];
        let mut curr = 0;
        let mut ans = 1;
        for n in nums.into_iter().skip(1) {
            let diff = n - *stack.last().unwrap();
            curr += diff * stack.len() as i32;
            stack.push(n);
            while curr > k {
                let first = stack.remove(0);
                curr -= n - first;
            }
            ans = ans.max(stack.len() as i32);
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::max_frequency(vec![3, 9, 6], 2));
}
