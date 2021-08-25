struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let h_degree = ((hour * 60 + minutes) % 720) as f64 * 0.5;
        let m_degree = minutes as f64 * 6 as f64;
        let diff = (h_degree - m_degree).abs();
        if diff > 180.0 {
            360.0 - diff
        } else {
            diff
        }
    }
}

fn main() {
    println!("{}", Solution::angle_clock(1, 57));
}
