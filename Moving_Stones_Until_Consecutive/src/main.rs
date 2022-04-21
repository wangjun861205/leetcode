struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut s = [a, b, c];
        s.sort();
        let [a, b, c] = s;
        let (min, max);
        if c - b == 1 && b - a == 1 {
            return vec![0, 0];
        }
        if c - b <= 2 || b - a <= 2 {
            min = 1;
        } else {
            min = 2;
        }
        max = c - b - 1 + b - a - 1;
        vec![min, max]
    }
}

fn main() {
    println!("Hello, world!");
}
