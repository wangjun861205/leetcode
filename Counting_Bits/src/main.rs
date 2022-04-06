struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 0..=n {
            ans[i as usize] = (i & 1) + ans[(i >> 1) as usize];
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::count_bits(2));
}
