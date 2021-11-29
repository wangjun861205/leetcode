struct Solution;

impl Solution {
    fn gen(n: i32, curr: i32, l: &mut Vec<i32>) {
        for i in 0..10 {
            if curr * 10 + i > 0 && curr * 10 + i <= n {
                l.push(curr * 10 + i);
                Solution::gen(n, curr * 10 + i, l);
            }
        }
    }
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        Solution::gen(n, 0, &mut ans);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::lexical_order(2));
}
