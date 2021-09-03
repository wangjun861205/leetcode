struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min_x = m;
        let mut min_y = n;
        for o in ops {
            min_x = min_x.min(o[0]);
            min_y = min_y.min(o[1]);
        }
        min_x * min_y
    }
}

fn main() {
    println!("Hello, world!");
}
