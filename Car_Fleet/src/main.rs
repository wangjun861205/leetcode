struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_by_key(|c| c.0);
        let mut prev_time =
            (target - cars.last().unwrap().0) as f64 / cars.last().unwrap().1 as f64;
        let mut count = 1;
        for i in (0..cars.len() - 1).rev() {
            let t = (target - cars[i].0) as f64 / cars[i].1 as f64;
            if t > prev_time {
                count += 1;
            }
            prev_time = prev_time.max(t);
        }
        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3])
    );
}
