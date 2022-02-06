struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        piles.reverse();
        let mut left = 1;
        let mut right = piles.len() - 1;
        let mut ans = 0;
        while left < right {
            ans += piles[left];
            left += 2;
            right -= 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
}
