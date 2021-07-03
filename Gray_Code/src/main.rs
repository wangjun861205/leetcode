struct Solution;

impl Solution {
    fn rc(n: i32, num: &mut i32, l: &mut Vec<i32>) {
        *num = *num ^ (1 << n);
        l.push(*num);
        for i in 0..n {
            Solution::rc(i, num, l);
        }
    }
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0];
        let mut num = 0;
        for i in 0..n {
            Solution::rc(i, &mut num, &mut ans)
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::gray_code(3));
}
