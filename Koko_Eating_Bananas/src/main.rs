struct Solution;

impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        piles.sort();
        let calc_total = |speed: i32| -> i32 {
            piles
                .iter()
                .map(|v| {
                    if v <= &speed {
                        1
                    } else {
                        if *v % speed == 0 {
                            *v / speed
                        } else {
                            *v / speed + 1
                        }
                    }
                })
                .sum()
        };
        let mut index = -1;
        for (i, v) in piles.iter().enumerate().rev() {
            let tail_sum: i32 = piles[i + 1..]
                .into_iter()
                .map(|vv| {
                    if *vv % *v == 0 {
                        *vv / *v
                    } else {
                        *vv / *v + 1
                    }
                })
                .sum();
            let hour = tail_sum + i as i32 + 1;
            if hour == h {
                return *v;
            } else if hour > h {
                index = i as i32;
                break;
            }
        }
        if index == -1 {
            let mut lo = 1;
            let mut hi = piles[0];
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let total_hour = calc_total(mid);
                if total_hour <= h {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            return lo;
        } else {
            let mut lo = piles[index as usize];
            let mut hi = piles[index as usize + 1];
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let total_hour = calc_total(mid);
                if total_hour <= h {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            return lo;
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_eating_speed(vec![1, 1, 1, 999999999], 10)
    );
}
