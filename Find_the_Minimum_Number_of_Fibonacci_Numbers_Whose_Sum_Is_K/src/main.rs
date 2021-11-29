struct Solution;

impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut fibs = vec![1, 1];
        while fibs[fibs.len() - 1] < k {
            let len = fibs.len();
            fibs.push(fibs[len - 1] + fibs[len - 2]);
        }
        let mut c = 0;
        while k > 0 {
            let f = fibs.pop().unwrap();
            if k >= f {
                k -= f;
                c += 1;
            }
        }
        c
    }
}

fn main() {
    println!("{}", Solution::find_min_fibonacci_numbers(7));
}
