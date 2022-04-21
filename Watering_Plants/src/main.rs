struct Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut water = capacity;
        let mut steps = 0;
        for i in 0..plants.len() {
            if water < plants[i] {
                steps += 2 * i;
                water = capacity;
            }
            steps += 1;
            water -= plants[i];
        }
        steps as i32
    }
}

fn main() {
    println!("{}", Solution::watering_plants(vec![1, 1, 1, 4, 2, 3], 4));
}
