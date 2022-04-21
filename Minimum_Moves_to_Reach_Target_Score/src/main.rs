struct Solution;

impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        if target == 1 {
            return 0;
        }
        if target % 2 == 0 {
            if max_doubles > 0 {
                return 1 + Solution::min_moves(target / 2, max_doubles - 1);
            }
            return target - 1;
        }
        return 1 + Solution::min_moves(target - 1, max_doubles);
    }
}

fn main() {
    println!("{}", Solution::min_moves(10, 4));
}
