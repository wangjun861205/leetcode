struct Solution;

impl Solution {
    fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        let d = a.max(b) / a.min(b);
        let m = a.max(b) % a.min(b);
        Solution::gcd(a.min(b), m)
    }

    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let ab = (a * b) / Solution::gcd(a, b);
        let ac = (a * c) / Solution::gcd(a, c);
        let bc = (b * c) / Solution::gcd(b, c);
        let abc = (a * (bc)) / Solution::gcd(a, bc);
        let mut lo = 1;
        let mut hi = 2000000000 as i64;
        let count = |v: i64| -> i64 {
            v / a as i64 + v / b as i64 + v / c as i64 - v / ab - v / ac - v / bc + v / abc
        };
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = count(mid);
            if cnt < n as i64 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::nth_ugly_number(1000000000, 2, 217983653, 336916467)
    );
}
