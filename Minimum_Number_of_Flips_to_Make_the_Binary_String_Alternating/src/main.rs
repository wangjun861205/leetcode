struct Solution;

impl Solution {
    fn shift(s: &mut String, even_zero: &mut i32, odd_one: &mut i32, even_one: &mut i32, odd_zero: &mut i32) {
        let c = s.remove(0);
        if c == '0' {
            *even_zero -= 1;
        } else {
            *even_one -= 1;
        }
        let mut temp = *even_zero;
        *even_zero = *odd_zero;
        *odd_zero = temp;
        temp = *even_one;
        *even_one = *odd_one;
        *odd_one = temp;
        if c == '0' {
            s.push(c);
            if s.len() % 2 == 0 {
                *odd_zero += 1;
            } else {
                *even_zero += 1;
            }
        } else {
            s.push(c);
            if s.len() % 2 == 0 {
                *odd_one += 1;
            } else {
                *even_one += 1;
            }
        }
    }
    pub fn min_flips(mut s: String) -> i32 {
        let length = s.len();
        let mut odd_zero = 0;
        let mut odd_one = 0;
        let mut even_zero = 0;
        let mut even_one = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                if i % 2 == 0 {
                    even_zero += 1;
                } else {
                    odd_zero += 1;
                }
            } else {
                if i % 2 == 0 {
                    even_one += 1;
                } else {
                    odd_one += 1;
                }
            }
        }
        let mut ans = i32::MAX;
        for _ in 0..length {
            ans = ans.min((odd_zero + even_one).min(odd_one + even_zero));
            Solution::shift(&mut s, &mut even_zero, &mut odd_one, &mut even_one, &mut odd_zero)
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_flips("01001001101".to_owned()));
}
