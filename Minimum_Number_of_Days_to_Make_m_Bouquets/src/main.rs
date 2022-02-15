struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() as i32 / k < m {
            return -1;
        }
        let mut max = *bloom_day.iter().max().unwrap();
        let mut min = 0;
        while min < max {
            let mid = (max + min) / 2;
            let mut total = 0;
            let mut count = 0;
            for v in &bloom_day {
                if v <= &mid {
                    count += 1;
                } else {
                    total += count / k;
                    count = 0;
                }
            }
            if count > 0 {
                total += count / k;
            }
            if total >= m {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        min
    }
}

fn main() {
    println!("{}", Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
}
