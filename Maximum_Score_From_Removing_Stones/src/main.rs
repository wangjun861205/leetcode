struct Solution;

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut l = vec![a, b, c];
        l.sort();
        if l[0] + l[1] <= l[2] {
            return l[0] + l[1];
        }
        let diff = l[0] + l[1] - l[2];
        diff / 2 + diff % 2 + l[0] + l[1] - (diff / 2 + diff % 2) * 2
    }
}
fn main() {
    println!("{}", Solution::maximum_score(4, 4, 6));
}
