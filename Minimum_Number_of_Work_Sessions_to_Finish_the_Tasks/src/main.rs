struct Solution;

// class Solution:
//     def minSessions(self, tasks: List[int], sessionTime: int) -> int:
//         n = len(tasks)

//         def clearBit(x, k):
//             return ~(1 << k) & x

//         @lru_cache(None)
//         def dp(mask, remainTime):
//             if mask == 0: return 0

//             ans = n  # There is up to N work sessions
//             for i in range(n):
//                 if (mask >> i) & 1:
//                     newMask = clearBit(mask, i)
//                     if tasks[i] <= remainTime:
//                         ans = min(ans, dp(newMask, remainTime - tasks[i]))  # Consume current session
//                     else:
//                         ans = min(ans, dp(newMask, sessionTime - tasks[i]) + 1)  # Create new session

//             return ans

//         return dp((1 << n) - 1, 0)

use std::collections::HashMap;

impl Solution {
    fn dp(tasks: &Vec<i32>, session_time: i32, remain_time: i32, bitset: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if bitset == 0 {
            return 0;
        }
        let mut ans = tasks.len() as i32;
        for i in 0..tasks.len() {
            if bitset & (1 << i) > 0 {
                let next_bitset = bitset ^ (1 << i);
                if tasks[i] <= remain_time {
                    ans = ans.min(if let Some(c) = cache.get(&(remain_time - tasks[i], next_bitset)) {
                        *c
                    } else {
                        Solution::dp(tasks, session_time, remain_time - tasks[i], next_bitset, cache)
                    })
                } else {
                    ans = ans.min(if let Some(c) = cache.get(&(session_time - tasks[i], next_bitset)) {
                        *c + 1
                    } else {
                        Solution::dp(tasks, session_time, session_time - tasks[i], next_bitset, cache) + 1
                    })
                }
            }
        }
        cache.insert((remain_time, bitset), ans);
        ans
    }
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let ans = Solution::dp(&tasks, session_time, 0, (1 << tasks.len()) - 1, &mut HashMap::new());
        ans
    }
}

fn main() {
    println!("{}", Solution::min_sessions(vec![1, 2, 3], 3));
    println!("{}", Solution::min_sessions(vec![3, 1, 3, 1, 1], 8));
    println!("{}", Solution::min_sessions(vec![1, 2, 3, 4, 5], 15));
    println!("{}", Solution::min_sessions(vec![5, 7, 5], 7));
    println!("{}", Solution::min_sessions(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 14));
    println!("{}", Solution::min_sessions(vec![2, 7, 1, 4, 7, 6, 3, 2, 8, 9, 1, 3, 5, 7], 9));
}
