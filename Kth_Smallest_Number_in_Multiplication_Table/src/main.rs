struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut low = 1;
        let mut high = m * n;
        while low < high {
            let mid = low + (high - low) / 2;
            let count = (1..=m).map(|r| (mid / r).min(n)).sum::<i32>();
            if count >= k {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }
}

fn main() {
    println!("{}", Solution::find_kth_number(2, 3, 6));
}
