struct Solution;

use std::collections::HashSet;

impl Solution {
    fn bfs(nums: &Vec<i32>, nexts: Vec<i32>, goal: i32, visited: &mut HashSet<i32>) -> i32 {
        if nexts.is_empty() {
            return -1;
        }
        let mut new_nexts: Vec<i32> = Vec::new();
        for n in nexts {
            if n == goal {
                return 0;
            }
            for i in 0..nums.len() {
                let add = n + nums[i];
                if add == goal {
                    return 1;
                }
                if (add <= 1000 && add >= 0) && !visited.contains(&add) {
                    visited.insert(add);
                    new_nexts.push(add);
                }
                let sub = n - nums[i];
                if sub == goal {
                    return 1;
                }
                if (sub <= 1000 && sub >= 0) && !visited.contains(&sub) {
                    visited.insert(sub);
                    new_nexts.push(sub);
                }
                let xor = n ^ nums[i];
                if xor == goal {
                    return 1;
                }
                if (xor <= 1000 && xor >= 0) && !visited.contains(&xor) {
                    visited.insert(xor);
                    new_nexts.push(xor);
                }
            }
        }
        if new_nexts.is_empty() {
            return -1;
        }
        let next = Solution::bfs(nums, new_nexts, goal, visited);
        if next > 0 {
            return next + 1;
        }
        next
    }

    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut visited = HashSet::new();
        visited.insert(start);
        Solution::bfs(&nums, vec![start], goal, &mut visited)
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_operations(
            vec![
                -821076380, -675066150, -306144249, 504919653, 716238043, -124990086, -428244973, 655635118, -685309701, -829855358, -383651019, -469183737, 481606536, 60542672, 70931791, 16572795,
                245816770, -764645310, 149691790, 350230253, 306994852, 189683672, 999272836, 811531837, -666576311, -612033029, 649577485
            ],
            495,
            -416969045
        )
    );
}
