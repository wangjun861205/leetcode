struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = *nums.first().unwrap();
        let left_less = nums.iter().fold(Vec::new(), |mut l, v| {
            if *v <= min {
                min = *v;
                l.push(false);
            } else {
                l.push(true)
            }
            l
        });
        let mut max = *nums.last().unwrap();
        let mut right_great = nums.iter().rev().fold(Vec::new(), |mut l, v| {
            if *v >= max {
                max = *v;
                l.push(false);
            } else {
                l.push(true);
            }
            l
        });
        right_great.reverse();
        left_less.into_iter().zip(right_great).any(|(a, b)| a && b)
    }
}

fn main() {
    println!("{}", Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
}
