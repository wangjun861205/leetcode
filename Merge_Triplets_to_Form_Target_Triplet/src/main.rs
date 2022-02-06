struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut firsts: Vec<Vec<i32>> = Vec::new();
        let mut seconds: Vec<Vec<i32>> = Vec::new();
        let mut thirds: Vec<Vec<i32>> = Vec::new();
        for t in triplets {
            if t[0] == target[0] && t[1] <= target[1] && t[2] <= target[2] {
                firsts.push(t.clone());
            }
            if t[1] == target[1] && t[0] <= target[0] && t[2] <= target[2] {
                seconds.push(t.clone());
            }
            if t[2] == target[2] && t[0] <= target[0] && t[1] <= target[1] {
                thirds.push(t.clone());
            }
        }
        !firsts.is_empty() && !seconds.is_empty() && !thirds.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
