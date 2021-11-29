struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        if n < 10 {
            return n;
        }
        let mut i = 0;
        let mut curr = 0_i64;
        let mut next = 9_i64;
        while next < n as i64 {
            i += 1;
            curr = next + 1;
            next += 9 * 10_i64.pow(i as u32) * (i + 1);
        }
        let start = 10_i64.pow(i as u32);
        let offset = (n as i64 - curr) / (i + 1);
        let index = (n as i64 - curr) % (i + 1);
        (start + offset)
            .to_string()
            .chars()
            .nth(index as usize)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap()
    }
}

fn main() {
    println!("{}", Solution::find_nth_digit(1000000000));
}
