struct Solution;

impl Solution {
    fn dp(n: i32, cache: &mut Vec<i32>) -> i32 {
        let sq = (n as f32).sqrt() as i32;
        if sq.pow(2) == n {
            cache[n as usize] = 1;
            return 1;
        }
        let mut left = 1;
        let mut right = n - 1;
        let mut ans = 10000;
        while left <= right {
            let lc = if cache[left as usize] > 0 {
                cache[left as usize]
            } else {
                Solution::dp(left, cache)
            };
            let rc = if cache[right as usize] > 0 {
                cache[right as usize]
            } else {
                Solution::dp(right, cache)
            };
            ans = ans.min(lc + rc);
            left += 1;
            right -= 1;
        }
        cache[n as usize] = ans;
        ans
    }
    pub fn num_squares(n: i32) -> i32 {
        let mut cache = vec![0; 10001];
        Solution::dp(n, &mut cache)
    }
}

fn main() {
    println!("{}", Solution::num_squares(9999));
}
