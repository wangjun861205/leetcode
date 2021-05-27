struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0];
        let mut num = 0;
        for i in 0..n {
            let mask = 1 << i;
            num = num ^ mask;
            ans.push(num);
            for j in (0..i).rev() {
                let mask = 1 << j;
                num = num ^ mask;
                ans.push(num);
            }
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::gray_code(4));
}
