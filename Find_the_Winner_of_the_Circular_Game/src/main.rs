struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut l: Vec<i32> = (0..n).into_iter().collect();
        let mut start: usize = 0;
        while l.len() > 1 {
            let end = (start + k as usize - 1) % l.len();
            l.remove(end);
            start = end;
        }
        l[0] + 1
    }
}
fn main() {
    println!("{}", Solution::find_the_winner(6, 5));
}
