struct Solution;

use std::i32::{MAX, MIN};

impl Solution {
    pub fn ctoi(digits: Vec<char>) -> i32 {
        digits.iter().collect::<String>().parse::<i32>().unwrap()
    }
    pub fn itoc(n: i32) -> Vec<char> {
        n.to_string().chars().collect()
    }
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if divisor == MIN {
            if dividend == MIN {
                return 1;
            } else {
                return 0;
            }
        }
        if divisor == 1 {
            return dividend;
        } else if divisor == -1 {
            if dividend == MIN {
                return MAX;
            } else {
                return -dividend;
            }
        }
        let mut remain: Vec<char> = Vec::new();
        let mut result: Vec<char> = Vec::new();
        let mut dds: Vec<char> = dividend.to_string().chars().collect();
        let d = divisor;
        if dividend < 0 {
            if divisor < 0 {
                while dds.len() > 0 {
                    let c = dds.remove(0);
                    remain.push(c);
                    if c == '-' {
                        let c = dds.remove(0);
                        remain.push(c);
                    }
                    let mut r = Solution::ctoi(remain.clone());
                    let mut res = 0;
                    while r - d <= 0 {
                        r -= d;
                        res += 1;
                    }
                    if r == 0 {
                        remain = vec!['-', '0'];
                    } else {
                        remain = Solution::itoc(r);
                    }
                    result.append(&mut Solution::itoc(res));
                }
            } else {
                while dds.len() > 0 {
                    let c = dds.remove(0);
                    remain.push(c);
                    if c == '-' {
                        let c = dds.remove(0);
                        remain.push(c);
                    }
                    let mut r = Solution::ctoi(remain.clone());
                    let mut res = 0;
                    while r + d <= 0 {
                        r += d;
                        res += 1;
                    }
                    if r == 0 {
                        remain = vec!['-', '0'];
                    } else {
                        remain = Solution::itoc(r);
                    }
                    result.append(&mut Solution::itoc(res));
                }
            }
        } else {
            if divisor < 0 {
                while dds.len() > 0 {
                    let c = dds.remove(0);
                    remain.push(c);
                    if c == '-' {
                        let c = dds.remove(0);
                        remain.push(c);
                    }
                    let mut r = Solution::ctoi(remain.clone());
                    let mut res = 0;
                    while r + d >= 0 {
                        r += d;
                        res += 1;
                    }
                    remain = Solution::itoc(r);
                    result.append(&mut Solution::itoc(res));
                }
            } else {
                while dds.len() > 0 {
                    let c = dds.remove(0);
                    remain.push(c);
                    if c == '-' {
                        let c = dds.remove(0);
                        remain.push(c);
                    }
                    let mut r = Solution::ctoi(remain.clone());
                    let mut res = 0;
                    while r - d >= 0 {
                        r -= d;
                        res += 1;
                    }
                    remain = Solution::itoc(r);
                    result.append(&mut Solution::itoc(res));
                }
            }
        }
        let ans = Solution::ctoi(result);
        if dividend > 0 && divisor > 0 || dividend < 0 && divisor < 0 {
            ans
        } else {
            -ans
        }
    }
}
fn main() {
    println!("{}", Solution::divide(-2147483648, 2));
}
