struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn parse_time(s: String) -> i32 {
        let hour: i32 = s.chars().collect::<Vec<char>>()[..2]
            .to_vec()
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        let minute: i32 = s.chars().collect::<Vec<char>>()[3..]
            .to_vec()
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        hour * 60 + minute
    }
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut pairs: Vec<(i32, String)> = key_time
            .into_iter()
            .zip(key_name)
            .map(|(t, v)| (Solution::parse_time(t), v))
            .collect();
        pairs.sort_by_key(|(t, n)| *t);
        let mut ans: HashSet<String> = HashSet::new();
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut queue: Vec<(i32, String)> = Vec::new();
        for (time, name) in pairs {
            if queue.is_empty() {
                queue.push((time, name.clone()));
                *map.entry(name).or_insert(0) += 1;
            } else {
                if time - queue[0].0 <= 60 {
                    queue.push((time, name.clone()));
                    if let Some(count) = map.get_mut(&name) {
                        if *count > 1 {
                            ans.insert(name);
                        }
                        *count += 1;
                    } else {
                        map.insert(name, 1);
                    }
                } else {
                    while !queue.is_empty() {
                        if time - queue[0].0 <= 60 {
                            break;
                        }
                        let (_, n) = queue.remove(0);
                        *map.get_mut(&n).unwrap() -= 1;
                    }
                    queue.push((time, name.clone()));
                    if let Some(count) = map.get_mut(&name) {
                        if *count > 1 {
                            ans.insert(name.clone());
                        }
                        *count += 1;
                    } else {
                        map.insert(name.clone(), 1);
                    }
                }
            }
        }
        let mut ans: Vec<String> = ans.into_iter().collect();
        ans.sort();
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
