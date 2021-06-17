struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(
        matchsticks: Vec<i32>,
        target: i32,
        current: i32,
        count: i32,
        cache: &mut HashMap<(usize, i32), bool>,
    ) -> bool {
        if count == 0 {
            return true;
        }
        let sum: i32 = matchsticks.iter().sum();
        for i in 0..matchsticks.len() {
            let mut sticks = matchsticks.clone();
            let s = sticks.remove(i);
            let sum: i32 = sticks.iter().sum();
            if let Some(c) = cache.get(&(sticks.len(), sum)) {
                if *c {
                    return true;
                }
            } else {
                // 如果达到了规定的边长，则重新开始计算另一条边
                if current + s == target {
                    if Solution::rc(sticks, target, 0, count - 1, cache) {
                        return true;
                    }
                // 如果加上还达不到规定的边长，则继续拼凑这一边
                } else if current + s < target {
                    if Solution::rc(sticks, target, current + s, count, cache) {
                        return true;
                    }
                }
            }
        }
        cache.insert((matchsticks.len(), sum), false);
        false
    }
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        // 目标边长
        let target = sum / 4;
        let mut cache = HashMap::new();
        Solution::rc(matchsticks, target, 0, 4, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3])
    );
}
