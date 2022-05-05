struct Solution;

use std::collections::HashSet;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        let c = a % b;
        Solution::gcd(b, c)
    }
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let gcd = Solution::gcd(a, b);
        if x % gcd != 0 {
            return -1;
        }
        let forbidden: HashSet<i32> = forbidden.into_iter().collect();
        let boundary = x.max(*forbidden.iter().max().unwrap()) + a + b;
        let mut stack = vec![(0, -1)];
        let mut visited: HashSet<(i32, i32)> = vec![(0, -1)].into_iter().collect();
        let mut steps = 0;
        loop {
            let mut new_stack = Vec::new();
            for (v, dir) in stack {
                if v == x {
                    return steps;
                }
                if v + a <= boundary
                    && !forbidden.contains(&(v + a))
                    && !visited.contains(&(v + a, 1))
                {
                    visited.insert((v + a, 1));
                    new_stack.push((v + a, 1));
                }
                if dir != -1
                    && v - b > 0
                    && !forbidden.contains(&(v - b))
                    && !visited.contains(&(v - b, -1))
                {
                    visited.insert((v - b, -1));
                    new_stack.push((v - b, -1));
                }
            }
            if new_stack.is_empty() {
                return -1;
            }
            steps += 1;
            stack = new_stack;
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11)
    );
}
