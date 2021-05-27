use std::collections::HashMap;

struct Solution;

impl Solution {
    fn _max_coins(nums: Vec<i32>, map: &mut HashMap<String, i32>) -> i32 {
        println!("{:?}", map);
        if nums.len() == 0 {
            return 0;
        }
        let mut max_coins: Vec<i32> = Vec::new();
        for (i, v) in nums.iter().enumerate() {
            let mut l = nums.clone();
            l.remove(i);
            let s: String = l
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let mut val = 0;
            if i == 0 {
                if i == nums.len() - 1 {
                    val = *v;
                } else {
                    val = v * nums[i + 1];
                }
            } else {
                if i == nums.len() - 1 {
                    val = v * nums[i - 1];
                } else {
                    val = v * nums[i - 1] * nums[i + 1];
                }
            }
            if map.contains_key(&s) {
                max_coins.push(map.get(&s).unwrap() + val)
            } else {
                let next_max = Solution::_max_coins(l, map);
                map.insert(s, next_max);
                max_coins.push(val + next_max);
            }
        }
        let max = *max_coins.iter().max().unwrap();
        let s: String = nums
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        map.insert(s, max);
        max
    }

    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut l: Vec<i32> = nums.iter().filter(|&v| *v > 0).map(|v| *v).collect();
        let mut coins = 0;
        while l.len() > 0 {
            let (max_index, max_value) = l.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
            if l.len() == 1 {
                coins += max_value;
                break;
            } else {
                if max_index == 0 {
                    if l.len() >= 3 {
                        coins += l[0] * l[1] * l[2];
                        l.remove(1);
                    } else {
                        coins += l[0] * l[1];
                        l.remove(1);
                    }
                } else if max_index == l.len() - 1 {
                    if l.len() >= 3 {
                        let len = l.len();
                        coins += l[len - 1] * l[len - 2] * l[len - 3];
                        l.remove(len - 2);
                    } else {
                        let len = l.len();
                        coins += l[len - 1] * l[len - 2];
                        l.remove(len - 2);
                    }
                } else {
                    if max_index >= 2 {
                        coins += l[max_index - 2] * l[max_index - 1] * l[max_index];
                        l.remove(max_index - 1);
                    } else {
                        coins += l[max_index - 1] * l[max_index];
                        l.remove(max_index - 1);
                    }
                    let len = l.len();
                    if max_index <= len - 3 {
                        coins += l[max_index] * l[max_index + 1] * l[max_index + 2];
                        l.remove(max_index + 1);
                    } else {
                        coins += l[max_index] * l[max_index + 1];
                        l.remove(max_index + 1);
                    }
                }
            }
        }
        coins
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_coins(vec![
            8, 2, 6, 8, 9, 8, 1, 4, 1, 5, 3, 0, 7, 7, 0, 4, 2, 2, 5
        ])
    );
}
