struct Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_owned();
        }
        let mut ans: Vec<char> = Vec::new();
        loop {
            if n == 1 {
                ans.insert(0, '1');
                break;
            } else if n == 0 {
                break;
            } else {
                let mut nn = n / -2;
                let mut remain = n % -2;
                if remain == -1 {
                    nn += 1;
                    remain = 1;
                }
                if remain == 0 {
                    ans.insert(0, '0');
                } else {
                    ans.insert(0, '1');
                }
                n = nn;
            }
        }
        ans.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::base_neg2(4));
}
