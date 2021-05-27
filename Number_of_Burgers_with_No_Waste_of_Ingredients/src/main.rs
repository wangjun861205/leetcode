struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let n = tomato_slices - 2 * cheese_slices;
        if n == 0 {
            if cheese_slices == 0 {
                return vec![0, 0];
            }
            return vec![0, cheese_slices];
        }
        if n % 2 == 1 || n < 0 {
            return vec![];
        }
        if n / 2 > cheese_slices {
            return vec![];
        }
        vec![n / 2, cheese_slices - n / 2]
    }
}
fn main() {
    println!("Hello, world!");
}
