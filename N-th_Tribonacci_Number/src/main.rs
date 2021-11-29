struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut l = vec![0, 1, 1];
        match n {
            0 => return 0,
            1 => return 1,
            2 => return 1,
            _ => {
                for i in 3..=n as usize {
                    l.push(l[i - 3] + l[i - 2] + l[i - 1]);
                }
                return *l.last().unwrap();
            }
        }
    }
}

fn main() {
    println!("{}", Solution::tribonacci(25));
}
