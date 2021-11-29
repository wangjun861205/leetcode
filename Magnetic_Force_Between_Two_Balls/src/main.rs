struct Solution;

impl Solution {
    fn count(position: &Vec<i32>, d: i32) -> i32 {
        let (mut ans, mut curr) = (1, position[0]);
        for i in 1..position.len() {
            if position[i] - curr >= d {
                ans += 1;
                curr = position[i];
            }
        }
        ans
    }
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();
        let (mut l, mut r) = (0, *position.last().unwrap() - *position.first().unwrap());
        while l < r {
            let mid = r - (r - l) / 2;
            if Solution::count(&position, mid) >= m {
                l = mid
            } else {
                r = mid - 1;
            }
        }
        l
    }
}

fn main() {
    println!("{}", Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2));
}
