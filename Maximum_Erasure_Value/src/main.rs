struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();
        let mut ans = 0;
        let mut max = 0;
        for n in nums {
            if !set.contains(&n) {
                set.insert(n);
                stack.push(n);
                max += n;
            } else {
                loop {
                    ans = ans.max(max);
                    let p = stack.remove(0);
                    set.remove(&p);
                    max -= p;
                    if p == n {
                        set.insert(n);
                        stack.push(n);
                        max += n;
                        break;
                    }
                }
            }
        }
        ans = ans.max(max);
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
    );
}
