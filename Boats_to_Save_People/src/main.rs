struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 0;
        people.sort();
        let mut left = 0_usize;
        let mut right = people.len() - 1;
        while left < right {
            if people[left] + people[right] > limit {
                right -= 1;
            } else {
                left += 1;
                right -= 1;
            }
            ans += 1;
        }
        if left == right {
            ans += 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![3, 2, 2, 1], 3))
}
