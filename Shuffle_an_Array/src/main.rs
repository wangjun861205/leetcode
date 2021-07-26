use rand::{thread_rng, Rng};

struct Solution {
    origin: Vec<i32>,
    list: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            origin: nums.clone(),
            list: nums,
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.origin.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        for i in 0..self.list.len() {
            let j: usize = rng.gen_range(i..self.list.len());
            let tmp = self.list[i];
            self.list[i] = self.list[j];
            self.list[j] = tmp;
        }
        self.list.clone()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
fn main() {
    println!("Hello, world!");
}
