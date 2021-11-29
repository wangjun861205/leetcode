struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut zero_count: i32 = 0;
        let mut one_count: i32 = 0;
        for c in s.chars() {
            if c == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        if (zero_count - one_count).abs() > 1 {
            return -1;
        }
        if zero_count == one_count {
            let mut c1 = 0;
            let mut c2 = 0;
            for (i, c) in s.chars().enumerate() {
                if i % 2 == 0 {
                    if c == '1' {
                        c1 += 1;
                    } else {
                        c2 += 1;
                    }
                } else {
                    if c == '0' {
                        c1 += 1;
                    } else {
                        c2 += 1;
                    }
                }
            }
            return c1.min(c2) / 2;
        }
        if zero_count > one_count {
            let mut count = 0;
            for (i, c) in s.chars().enumerate() {
                if i % 2 == 0 {
                    if c == '1' {
                        count += 1;
                    }
                } else {
                    if c == '0' {
                        count += 1;
                    }
                }
            }
            return count / 2;
        } else {
            let mut count = 0;
            for (i, c) in s.chars().enumerate() {
                if i % 2 == 0 {
                    if c == '0' {
                        count += 1;
                    }
                } else {
                    if c == '1' {
                        count += 1;
                    }
                }
            }
            return count / 2;
        }
    }
}

fn main() {
    println!("{}", Solution::min_swaps("1110".to_owned()));
}
