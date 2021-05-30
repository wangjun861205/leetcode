struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut l = vec![0; num_people as usize];
        let mut amount = 1;
        while candies > 0 {
            l[((amount - 1) % num_people) as usize] += candies.min(amount);
            candies -= amount;
            amount += 1;
        }
        l
    }
}

fn main() {
    println!("{:?}", Solution::distribute_candies(7, 4));
}
