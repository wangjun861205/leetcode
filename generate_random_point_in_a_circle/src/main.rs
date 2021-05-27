extern crate rand;
use std::f64::consts::PI;
struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            radius: radius,
            x_center: x_center,
            y_center: y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let a = rand::random::<f64>() * 2.0 * PI;
        let r = rand::random::<f64>().sqrt() * self.radius;
        vec![r * a.cos() + self.x_center, r * a.sin() + self.y_center]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

fn main() {
    let s = Solution::new(1_f64, 0_f64, 0_f64);
    let v = s.rand_point();
    println!("{:?}", v);
    assert!(v[0].powi(2) + v[1].powi(2) < 1_f64);
}
