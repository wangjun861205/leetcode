struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(mut left: String, mut right: String, target: i32, cache: &mut HashMap<(usize, i32), Vec<String>>) -> Vec<String> {
        let mut ans = Vec::new();
        if !left.is_empty() {
            let lv = left.parse::<i32>().unwrap();
            if lv == target {
                ans.push(left.clone());
            }
        }
        if right.is_empty() {
            if !left.is_empty() {
                if left.parse::<i32>().unwrap() == target {
                    return vec![left];
                }
            }
            return Vec::new();
        }
        let c = right.remove(0);
        left.push(c);
        let lv = left.parse::<i32>().unwrap();
        let add_target = target - lv;
        let nexts = if let Some(c) = cache.get(&(right.len(), target)) {
            c.clone()
        } else {
            Solution::dp("".to_owned(), right.clone(), add_target, cache)
        };
        for next in nexts {
            ans.push(format!("{}+{}", left, next));
        }
        let sub_target = lv - target;
        let nexts = if let Some(c) = cache.get(&(right.len(), target)) {
            c.clone()
        } else {
            Solution::dp("".to_owned(), right.clone(), sub_target, cache)
        };
        for next in nexts {
            ans.push(format!("{}-{}", left, next));
        }
        for i in 1..=right.len() {
            let rv = right.chars().take(i).collect::<String>().parse::<i32>().unwrap();
            let add_nexts = if let Some(c) = cache.get(&(right.len() - i, target - rv * lv)) {
                c.clone()
            } else {
                Solution::dp("".to_owned(), right.chars().skip(i).collect(), target - rv * lv, cache)
            };
            for next in add_nexts {
                ans.push(format!("{}*{}+{}", lv, rv, next));
            }
            let sub_nexts = if let Some(c) = cache.get(&(right.len() - i, lv * rv - target)) {
                c.clone()
            } else {
                Solution::dp("".to_owned(), right.chars().skip(i).collect(), lv * rv - target, cache)
            };
            for next in sub_nexts {
                ans.push(format!("{}*{}-{}", lv, rv, next));
            }
        }
        ans.append(&mut Solution::dp(left.clone(), right.clone(), target, cache));
        cache.insert((left.len() + right.len(), target), ans.clone());
        ans
    }
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut cache = HashMap::new();
        Solution::dp("".to_owned(), num, target, &mut cache)
    }
}
fn main() {
    println!("{:?}", Solution::add_operators("232".to_owned(), 8));
}
