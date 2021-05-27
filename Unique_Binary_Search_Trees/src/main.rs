struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        match n {
            0 | 1 => return 1,
            2 => return 2,
            _ => {
                let mut ans = 0;
                for i in 1..=n {
                    let left = Solution::num_trees(i - 1);
                    let right = Solution::num_trees(n - i);
                    ans += left * right;
                }
                return ans;
            }
        }
    }
}

fn main() {
    println!("{}", Solution::num_trees(1));
}
