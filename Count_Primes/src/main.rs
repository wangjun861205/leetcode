struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize];
        for i in 2..n as usize {
            if !is_prime[i] {
                continue;
            }
            let mut j = i * i;
            while j < n as usize {
                is_prime[j] = false;
                j += i;
            }
        }
        is_prime.into_iter().skip(2).filter(|v| *v).count() as i32
    }
}
fn main() {
    println!("{}", Solution::count_primes(10));
}
