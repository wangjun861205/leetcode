struct Solution;

// 1, 1, 2, 3, 2, 1, 1
// 0, 1, 2, 3, 4, 5, 6

// 1, 2, 3, 2, 1
// 0, 1, 2, 3, 4

// 2, 3, 4, 3, 2
// 0, 1, 2, 3, 4

impl Solution {
    fn calc(n: i64, index: i64, mid: i64) -> i64 {
        let left = if mid > index + 1 {
            (mid + (mid - index)) * (index + 1) / 2
        } else {
            mid * (mid + 1) / 2 + (index + 1 - mid)
        };
        let right = if mid > n - index {
            (mid + mid - (n - index - 1)) * (n - index) / 2
        } else {
            mid * (mid + 1) / 2 + (n - index - mid)
        };
        left + right - mid
    }
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut l = 1;
        let mut r = max_sum as i64;
        while l < r {
            let m = (l + r) / 2;
            let sum = Solution::calc(n as i64, index as i64, m);
            if sum > max_sum as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if Solution::calc(n as i64, index as i64, l) > max_sum as i64 {
            l as i32 - 1
        } else {
            l as i32
        }
    }
}
fn main() {
    println!("{}", Solution::max_value(1, 0, 1));
}
