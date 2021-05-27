struct Solution;

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![1];
        'outer: while ans.len() < n as usize {
            let mut odd: Vec<i32> = Vec::new();
            for v in ans.iter() {
                if v * 2 - 1 <= n {
                    odd.push(2 * v - 1);
                }
            }
            let mut even: Vec<i32> = Vec::new();
            for v in ans.iter() {
                if v * 2 <= n {
                    odd.push(2 * v);
                }
            }
            odd.append(&mut even);
            ans = odd;
        }
        ans[..n as usize].to_vec()
    }
}
fn main() {
    println!("{:?}", Solution::beautiful_array(10));
}
