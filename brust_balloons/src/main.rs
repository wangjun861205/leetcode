struct Solution;

// class Solution:
//     def maxCoins(self, nums):
//         A = [1] + nums + [1]

//         @lru_cache(None)
//         def dfs(i, j):
//             return max([A[i]*A[k]*A[j] + dfs(i,k) + dfs(k,j) for k in range(i+1, j)] or [0])

//         return dfs(0, len(A) - 1)

use std::collections::HashMap;

impl Solution {
    fn dp(nums: &Vec<i32>, left: usize, right: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if left == right {
            return 0;
        }
        let mut ans = 0;
        for i in left + 1..right {
            let v = nums[i] * nums[left] * nums[right];
            let l = if let Some(c) = cache.get(&(left, i)) { *c } else { Solution::dp(nums, left, i, cache) };
            let r = if let Some(c) = cache.get(&(i, right)) { *c } else { Solution::dp(nums, i, right, cache) };
            ans = ans.max(v + l + r);
        }
        cache.insert((left, right), ans);
        ans
    }

    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.insert(0, 1);
        nums.push(1);
        Solution::dp(&nums, 0, nums.len() - 1, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::max_coins(vec![3, 1, 5, 8]));
}
