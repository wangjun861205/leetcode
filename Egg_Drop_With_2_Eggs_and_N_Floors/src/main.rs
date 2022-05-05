struct Solution;

impl Solution {
    fn dp(n: i32, cache: &mut Vec<i32>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut ans = i32::MAX;
        for i in 1..=n {
            let broken = i;
            let survive = if cache[(n - i) as usize] > 0 {
                cache[(n - i) as usize] + 1
            } else {
                Solution::dp(n - i, cache) + 1
            };
            ans = ans.min(broken.max(survive));
        }
        cache[n as usize] = ans;
        ans
    }

    pub fn two_egg_drop(n: i32) -> i32 {
        Solution::dp(n, &mut vec![0; n as usize + 1])
    }
}

fn main() {
    println!("{}", Solution::two_egg_drop(100));
}
